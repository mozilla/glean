/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import org.apache.tools.ant.taskdefs.condition.Os
import org.gradle.api.GradleException
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.Task
import org.gradle.api.artifacts.transform.ArtifactTransform
import org.gradle.api.internal.artifacts.ArtifactAttributes
import org.gradle.api.tasks.Exec
import org.gradle.api.tasks.TaskProvider

// The suppression "GrPackage" is needed below since Android Studio wants this file to have
// a package name, but adding one causes the build to fail with:
//    "'.../GleanGradlePlugin.groovy' should not contain a package statement"
// due to how this file is included directly in the local build.

/*
 * A helper class to extract metrics.yaml files from AAR files.
 */
@SuppressWarnings("GrPackage")
class GleanMetricsYamlTransform extends ArtifactTransform {
    List<File> transform(File file) {
        def f = new File(file, "metrics.yaml")
        if (f.exists()) {
            return [f]
        }
        return []
    }
}

@SuppressWarnings("GrPackage")
class GleanPlugin implements Plugin<Project> {
    // The version of glean_parser to install from PyPI.
    private String GLEAN_PARSER_VERSION = "1.20.3"
    // The version of Miniconda is explicitly specified.
    // Miniconda3-4.5.12 is known to not work on Windows.
    private String MINICONDA_VERSION = "4.5.11"

    private String TASK_NAME_PREFIX = "gleanGenerateMetrics"

    /* This script runs a given Python module as a "main" module, like
     * `python -m module`. However, it first checks that the installed
     * package is at the desired version, and if not, upgrades it using `pip`.
     */
    String runPythonScript = """
import importlib
import subprocess
import sys
module_name = sys.argv[1]
expected_version = sys.argv[2]
try:
    module = importlib.import_module(module_name)
except ImportError:
    found_version = None
else:
    found_version = getattr(module, '__version__')
if found_version != expected_version:
    subprocess.check_call([
        sys.executable,
        '-m',
        'pip',
        'install',
        '--upgrade',
        f'{module_name}=={expected_version}'
    ])
subprocess.check_call([
    sys.executable,
    '-m',
    module_name
] + sys.argv[3:])
"""

    static File getPythonCommand(File condaDir) {
        // Note that the command line is OS dependant: on linux/mac is Miniconda3/bin/python.
        if (Os.isFamily(Os.FAMILY_WINDOWS)) {
            return new File(condaDir, "python")
        }

        return new File(condaDir, "bin/python")
    }

    /*
     * Get the list of metrics.yaml and pings.yaml files we should use.
     */
    def getYamlFiles(Project project) {
        if (project.ext.has("gleanYamlFiles")) {
            return project.ext.gleanYamlFiles
        } else {
            return [
                "${project.projectDir}/metrics.yaml",
                "${project.projectDir}/pings.yaml"
            ]
        }
    }

    /*
     * Adds tasks that generates the Glean metrics API for a project.
     */
    def setupTasks(Project project, File condaDir) {
        return { variant ->
            def sourceOutputDir = "${project.buildDir}/generated/source/glean/${variant.dirName}/kotlin"
            // Get the name of the package as if it were to be used in the R or BuildConfig
            // files. This is required since applications can define different application ids
            // depending on the variant type: the generated API definitions don't need to be
            // different due to that.
            TaskProvider buildConfigProvider = variant.getGenerateBuildConfigProvider()
            def originalPackageName = buildConfigProvider.get().getBuildConfigPackageName()

            def fullNamespace = "${originalPackageName}.GleanMetrics"
            def generateKotlinAPI = project.task("${TASK_NAME_PREFIX}SourceFor${variant.name.capitalize()}", type: Exec) {
                description = "Generate the Kotlin code for the Metrics API"

                if (project.ext.has("allowMetricsFromAAR")) {
                    // This is sufficiently lazy to be valid at configuration time.  See the model at
                    // https://github.com/google/protobuf-gradle-plugin/blob/6d99a421c8d15710045e4e8d31a3af6cb0cc3b70/src/main/groovy/com/google/protobuf/gradle/ProtobufPlugin.groovy#L270-L277
                    inputs.files variant.compileConfiguration.incoming.artifactView {
                        attributes {
                            it.attribute(ArtifactAttributes.ARTIFACT_FORMAT, 'glean-metrics-yaml')
                        }
                    }.files
                }

                // Add local registry files as input to this task. They will be turned
                // into `arg`s later.
                for (String item : getYamlFiles(project)) {
                    if (project.file(item).exists()) {
                        inputs.file item
                    }
                }

                outputs.dir sourceOutputDir

                workingDir project.rootDir
                commandLine getPythonCommand(condaDir)

                def gleanNamespace = "mozilla.components.service.glean"
                if (project.ext.has("gleanNamespace")) {
                    gleanNamespace = project.ext.get("gleanNamespace")
                }

                args "-c"
                args runPythonScript
                args "glean_parser"
                args GLEAN_PARSER_VERSION
                args "translate"
                args "-f"
                args "kotlin"
                args "-o"
                args "$sourceOutputDir"
                args "-s"
                args "namespace=$fullNamespace"
                args "-s"
                args "glean_namespace=$gleanNamespace"

                // If we're building the Glean library itself (rather than an
                // application using Glean) pass the --allow-reserved flag so we can
                // use metrics in the "glean..." category
                if (project.ext.has("allowGleanInternal")) {
                    args "--allow-reserved"
                }

                doFirst {
                    // Add the potential 'metrics.yaml' files at evaluation-time, rather than
                    // configuration-time. Otherwise the Gradle build will fail.
                    inputs.files.forEach { file ->
                        project.logger.lifecycle("Glean SDK - generating API from ${file.path}")
                        args file.path
                    }
                }

                // Only show the output if something went wrong.
                ignoreExitValue = true
                standardOutput = new ByteArrayOutputStream()
                errorOutput = standardOutput
                doLast {
                    if (execResult.exitValue != 0) {
                        throw new GradleException("Process '${commandLine}' finished with non-zero exit value ${execResult.exitValue}:\n\n${standardOutput.toString()}")
                    }
                }
            }

            def generateGleanMetricsDocs = project.task("${TASK_NAME_PREFIX}DocsFor${variant.name.capitalize()}", type: Exec) {
                description = "Generate the Markdown docs for the collected metrics"

                def gleanDocsDirectory = "${project.projectDir}/docs"
                if (project.ext.has("gleanDocsDirectory")) {
                    gleanDocsDirectory = project.ext.get("gleanDocsDirectory")
                }

                if (project.ext.has("allowMetricsFromAAR")) {
                    // This is sufficiently lazy to be valid at configuration time.  See the model at
                    // https://github.com/google/protobuf-gradle-plugin/blob/6d99a421c8d15710045e4e8d31a3af6cb0cc3b70/src/main/groovy/com/google/protobuf/gradle/ProtobufPlugin.groovy#L270-L277
                    inputs.files variant.compileConfiguration.incoming.artifactView {
                        attributes {
                            it.attribute(ArtifactAttributes.ARTIFACT_FORMAT, 'glean-metrics-yaml')
                        }
                    }.files
                }

                // Add local registry files as input to this task. They will be turned
                // into `arg`s later.
                for (String item : getYamlFiles(project)) {
                    if (project.file(item).exists()) {
                        inputs.file item
                    }
                }

                outputs.dir gleanDocsDirectory
                workingDir project.rootDir
                commandLine getPythonCommand(condaDir)

                args "-c"
                args runPythonScript
                args "glean_parser"
                args GLEAN_PARSER_VERSION
                args "translate"
                args "-f"
                args "markdown"
                args "-o"
                args gleanDocsDirectory

                // If we're building the Glean library itself (rather than an
                // application using Glean) pass the --allow-reserved flag so we can
                // use metrics in the "glean..." category
                if (project.ext.has("allowGleanInternal")) {
                    args "--allow-reserved"
                }

                doFirst {
                    // Add the potential 'metrics.yaml' files at evaluation-time, rather than
                    // configuration-time. Otherwise the Gradle build will fail.
                    inputs.files.forEach{ file ->
                        project.logger.lifecycle("Glean SDK - generating docs for ${file.path} in $gleanDocsDirectory")
                        args file.path
                    }
                }

                // Only show the output if something went wrong.
                ignoreExitValue = true
                standardOutput = new ByteArrayOutputStream()
                errorOutput = standardOutput
                doLast {
                    if (execResult.exitValue != 0) {
                        throw new GradleException("Process '${commandLine}' finished with non-zero exit value ${execResult.exitValue}:\n\n${standardOutput.toString()}")
                    }
                }
            }

            // Only attach the generation task if the metrics file is available or we're requested
            // to fetch them from AAR files. We don't need to fail hard otherwise, as some 3rd party
            // project might just want metrics included in Glean and nothing more.
            def yamlFileExists = false
            for (String item : getYamlFiles(project)) {
                if (project.file(item).exists()) {
                    yamlFileExists = true
                    break
                }
            }

            if (yamlFileExists
                || project.ext.has("allowMetricsFromAAR")) {
                // Generate the metrics docs, if requested
                if (project.ext.has("gleanGenerateMarkdownDocs")) {
                    generateKotlinAPI.dependsOn(generateGleanMetricsDocs)
                }

                // This is an Android-Gradle plugin 3+-ism.  Culted from reading the source,
                // searching for "registerJavaGeneratingTask", and finding
                // https://github.com/GoogleCloudPlatform/endpoints-framework-gradle-plugin/commit/2f2b91476fb1c6647791e2c6fe531a47615a1e85.
                // The added directory doesn't appear in the paths listed by the
                // `sourceSets` task, for reasons unknown.
                variant.registerJavaGeneratingTask(generateKotlinAPI, new File(sourceOutputDir))
            }
        }
    }

    File setupPythonEnvironmentTasks(Project project) {
        // This sets up tasks to install a Miniconda3 environment. It installs
        // into the gradle user home directory so that it will be shared between
        // all libraries that use Glean. This is important because it is
        // approximately 300MB in installed size.
        File condaBootstrapDir = new File(
            project.getGradle().gradleUserHomeDir,
            "glean/bootstrap-${MINICONDA_VERSION}"
        )
        File condaDir = new File(
            condaBootstrapDir,
            "Miniconda3"
        )

        // Even though we are installing the Miniconda environment to the gradle user
        // home directory, the gradle-python-envs plugin is hardcoded to download the
        // installer to the project's build directory. Doing so will fail if the
        // project's build directory doesn't already exist. This task ensures that
        // the project's build directory exists before downloading and installing the
        // Miniconda environment.
        // See https://github.com/JetBrains/gradle-python-envs/issues/26
        // The fix in the above is not actually sufficient -- we need to add createBuildDir
        // as a dependency of Bootstrap_CONDA (where conda is installed), as the preBuild
        // task alone isn't early enough.
        Task createBuildDir = project.task("createBuildDir") {
            description = "Make sure the build dir exists before creating the Python Environments"
            onlyIf {
                !project.file(project.buildDir).exists()
            }
            doLast {
                project.logger.lifecycle("Creating build directory:" + project.buildDir.getPath())
                project.buildDir.mkdir()
            }
        }

        // Configure the Python environments
        project.envs {
            bootstrapDirectory = condaBootstrapDir
            pipInstallOptions = "--trusted-host pypi.python.org --no-cache-dir"

            // Setup a miniconda environment. conda is used because it works
            // non-interactively on Windows, unlike the standard Python installers
            conda "Miniconda3", "Miniconda3-${MINICONDA_VERSION}", "64", ["glean_parser==${GLEAN_PARSER_VERSION}"]
        }

        project.tasks.whenTaskAdded { task ->
            if (task.name.startsWith('Bootstrap_CONDA')) {
                task.dependsOn(createBuildDir)
            }
        }
        project.preBuild.dependsOn(createBuildDir)
        project.preBuild.finalizedBy("build_envs")

        return condaDir
    }

    void setupExtractMetricsFromAARTasks(Project project) {
        // Support for extracting metrics.yaml from artifact files.

        // This is how to extract `metrics.yaml` and `pings.yaml` from AAR files: an "artifact transform"
        // identifies the files in an "exploded AAR" directory.  See
        // https://docs.gradle.org/current/userguide/dependency_management_attribute_based_matching.html#sec:abm_artifact_transforms.
        // This is exactly how elements of AAR files are consumed by the Android-Gradle plugin; see the
        // transforms defined in
        // https://android.googlesource.com/platform/tools/base/+/studio-master-dev/build-system/gradle-core/src/main/java/com/android/build/gradle/internal/dependency/AarTransform.java
        // and their usage at
        // https://android.googlesource.com/platform/tools/base/+/studio-master-dev/build-system/gradle-core/src/main/java/com/android/build/gradle/internal/VariantManager.java#592.
        //
        // Note that this mechanism only applies to `module` dependencies (i.e., AAR files downloaded from
        // Maven) and not to `project` dependencies in the same root project or substituted as part of a
        // Gradle composite build.
        if (project.ext.has("allowMetricsFromAAR")) {
            project.dependencies {
                registerTransform { reg ->
                    // The type here should be
                    // `com.android.build.gradle.internal.publishing.AndroidArtifacts.ArtifactType.EXPLODED_AAR.getType())`,
                    // but there's no good way to access the including script's classpath from `apply from:`
                    // scripts. See https://stackoverflow.com/a/37060550. The 'android-exploded-aar' string is
                    // very unlikely to change, so it's just hard-coded.
                    reg.getFrom().attribute(
                            ArtifactAttributes.ARTIFACT_FORMAT,
                            'android-exploded-aar')
                    reg.getTo().attribute(
                            ArtifactAttributes.ARTIFACT_FORMAT,
                            'glean-metrics-yaml')
                    reg.artifactTransform(GleanMetricsYamlTransform.class)
                }
            }
        }
    }

    void apply(Project project) {
        project.ext.glean_version = "29.0.0"

        File condaDir = setupPythonEnvironmentTasks(project)
        project.ext.set("gleanCondaDir", condaDir)

        setupExtractMetricsFromAARTasks(project)

        if (project.android.hasProperty('applicationVariants')) {
            project.android.applicationVariants.all(setupTasks(project, condaDir))
        } else {
            project.android.libraryVariants.all(setupTasks(project, condaDir))
        }
    }
}

// Put an instance of the plugin in ext so it can be used from the outside
// by Glean's own projects. This is not used by third-parties when using the
// plugin.
ext.glean_plugin = new GleanPlugin()
