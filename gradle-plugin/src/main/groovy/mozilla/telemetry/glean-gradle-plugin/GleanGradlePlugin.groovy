/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import org.apache.tools.ant.taskdefs.condition.Os
import org.gradle.api.GradleException
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.Task
import org.gradle.api.artifacts.transform.TransformAction
import org.gradle.api.artifacts.transform.TransformParameters
import org.gradle.api.artifacts.transform.TransformOutputs
import org.gradle.api.artifacts.transform.InputArtifact
import org.gradle.api.provider.Provider
import org.gradle.api.file.FileSystemLocation
import org.gradle.api.artifacts.ComponentMetadataRule
import org.gradle.api.artifacts.ComponentMetadataContext
import org.gradle.api.artifacts.component.ModuleComponentIdentifier
import org.gradle.api.artifacts.type.ArtifactTypeDefinition
import org.gradle.api.tasks.Exec
import org.gradle.api.tasks.OutputDirectory
import org.gradle.api.tasks.PathSensitive
import org.gradle.api.tasks.PathSensitivity
import org.gradle.api.tasks.TaskProvider
import org.gradle.api.file.DirectoryProperty
import org.gradle.work.DisableCachingByDefault

import groovy.transform.CompileStatic

// The suppression "GrPackage" is needed below since Android Studio wants this file to have
// a package name, but adding one causes the build to fail with:
//    "'.../GleanGradlePlugin.groovy' should not contain a package statement"
// due to how this file is included directly in the local build.

// Exec task with a wired output dir, required to register it via addGeneratedSourceDirectory.
@SuppressWarnings("GrPackage")
@DisableCachingByDefault(because = "Shells out to glean_parser, whose environment is not tracked as an input")
abstract class GenerateGleanMetricsAPITask extends Exec {
    @OutputDirectory
    abstract DirectoryProperty getOutputDir()
}

/*
 * A helper class to extract metrics.yaml files from AAR files.
 */
@SuppressWarnings("GrPackage")
@DisableCachingByDefault(because = "Extracting a single file is cheaper than caching the result")
abstract class GleanMetricsYamlTransform implements TransformAction<TransformParameters.None> {
    @InputArtifact
    @PathSensitive(PathSensitivity.RELATIVE)
    abstract Provider<FileSystemLocation> getInputArtifact()

    @Override
    void transform(TransformOutputs outputs) {
        def file = inputArtifact.get().asFile
        def f = new File(file, "metrics.yaml")
        if (f.exists()) {
            outputs.file(f)
        }
    }
}

@SuppressWarnings("GrPackage")
class GleanPlugin implements Plugin<Project> {
    // The version of glean_parser to install from PyPI.
    private String GLEAN_PARSER_VERSION = "20.1"

    private String TASK_NAME_PREFIX = "gleanGenerateMetrics"

    /* This script runs a given Python module as a "main" module, like
     * `python -m module`. However, it first checks that the installed
     * package is at the desired version, and if not, upgrades it using `pip`.
     *
     * Note: Groovy doesn't support embedded " in multi-line strings, so care
     * should be taken to use ' everywhere in this code snippet.
     */
    String runPythonScript = """
import importlib
import subprocess
import sys
offline = sys.argv[1] == 'offline'
module_name = sys.argv[2]
expected_version = sys.argv[3]
try:
    module = importlib.import_module(module_name)
except ImportError:
    found_version = None
else:
    found_version = getattr(module, '__version__')

if not offline:
    # When running in online mode, we always install.
    # If it is installed this is essentially a no-op,
    # otherwise it installs/upgrades.
    if 'git' in expected_version:
        target=expected_version
    else:
        target=f'{module_name}~={expected_version}'

    subprocess.check_call([
        sys.executable,
        '-m',
        'pip',
        'install',
        '--upgrade',
        target
    ])
else:
    error_text = f'''
    Using Python environment at {sys.executable},
    expected glean_parser version ~={expected_version}, found {found_version}.
    Please remove the Python environment, then prepare the package wheels for use:

      mkdir -p glean-wheels
      cd glean-wheels
      pip download glean_parser~={expected_version}
    '''

    if found_version is None:
        print(error_text)
        sys.exit(1)
    else:
        # We check MAJOR.MINOR only
        expected_ver = expected_version.split('.')
        expected_maj, expected_min = int(expected_ver[0]), int(expected_ver[1])
        current_ver = found_version.split('.')
        current_maj, current_min = int(current_ver[0]), int(current_ver[1])

        if current_maj > expected_maj or current_maj < expected_maj or (current_maj == expected_maj and current_min < expected_min):
            print(error_text)
            sys.exit(1)
try:
    subprocess.check_call([
        sys.executable,
        '-m',
        module_name
    ] + sys.argv[4:])
except:
    # We don't need to show a traceback in this helper script.
    # Only the output of the subprocess is interesting.
    sys.exit(1)
"""

    // Are we doing an offline build (by passing `--offline` to `./gradle`)?
    private Boolean isOffline

    // Null when the caller supplies their own environment via gleanPythonEnvDir.
    private TaskProvider<Exec> installGleanParser

    static File getPythonCommand(File envDir) {
        // Note that the command line is OS dependant.
        if (Os.isFamily(Os.FAMILY_WINDOWS)) {
            return new File(envDir, "Scripts\\python")
        }

        return new File(envDir, "bin/python")
    }

    /*
     * Get the list of metrics.yaml and pings.yaml files we should use.
     */
    static def getYamlFiles(Project project) {
        if (project.ext.has("gleanYamlFiles")) {
            return project.ext.gleanYamlFiles
        } else {
            return [
                "${project.projectDir}/metrics.yaml",
                "${project.projectDir}/pings.yaml",
                "${project.projectDir}/tags.yaml"
            ]
        }
    }

    /*
     * Adds tasks that generates the Glean metrics API for a project.
     */
    def setupTasks(Project project, File envDir, boolean isApplication, String parserVersion) {
        return { variant ->
            // Get the name of the package as if it were to be used in the R or BuildConfig
            // files. This is required since applications can define different application ids
            // depending on the variant type: the generated API definitions don't need to be
            // different due to that.
            def namespaceProvider = variant.namespace.map({ ns -> "namespace=${ns}.GleanMetrics" })

            def generateKotlinAPI = project.tasks.register("${TASK_NAME_PREFIX}SourceFor${variant.name.capitalize()}", GenerateGleanMetricsAPITask) {
                if (installGleanParser != null) {
                    dependsOn(installGleanParser)
                }
                description = "Generate the Kotlin code for the Metrics API"

                if (project.ext.has("allowMetricsFromAAR")) {
                    // This is sufficiently lazy to be valid at configuration time.  See the model at
                    // https://github.com/google/protobuf-gradle-plugin/blob/6d99a421c8d15710045e4e8d31a3af6cb0cc3b70/src/main/groovy/com/google/protobuf/gradle/ProtobufPlugin.groovy#L270-L277
                    inputs.files variant.compileConfiguration.incoming.artifactView {
                        attributes {
                            it.attribute(ArtifactTypeDefinition.ARTIFACT_TYPE_ATTRIBUTE, 'glean-metrics-yaml')
                        }
                    }.files
                }

                // Add local registry files as input to this task.
                // They will be turned into `arg`s later.
                inputs.files(getYamlFiles(project)).optional(true)

                workingDir project.rootDir
                commandLine getPythonCommand(envDir)

                def gleanNamespace = "mozilla.components.service.glean"
                if (project.ext.has("gleanNamespace")) {
                    gleanNamespace = project.ext.get("gleanNamespace")
                }

                args "-c"
                args runPythonScript
                args isOffline ? "offline" : "online"
                args "glean_parser"
                args parserVersion
                args "translate"
                args "--allow-missing-files"
                args "-f"
                args "kotlin"
                args "-s"
                args "glean_namespace=$gleanNamespace"

                // If we're building the Glean library itself (rather than an
                // application using Glean) pass the --allow-reserved flag so we can
                // use metrics in the "glean..." category
                if (project.ext.has("allowGleanInternal")) {
                    args "--allow-reserved"
                }

                // Only generate build info for applications, not for libraries.
                // From android-gradle 7.0 on the `VERSION_CODE` and `VERSION_NAME` fields
                // are not set for libraries anymore
                if (!isApplication) {
                    args "-s"
                    args "with_buildinfo=false"
                } else {
                    // For applications check if they overwrote the build date.
                    if (project.ext.has("gleanBuildDate")) {
                        args "-s"
                        args "build_date=${project.ext.get("gleanBuildDate")}"
                    }
                }

                // Enable expiration by major version, if a major version is provided.
                if (project.ext.has("gleanExpireByVersion")) {
                    args "--expire-by-version=${project.ext.get("gleanExpireByVersion")}"
                }

                doFirst {
                    args "-o"
                    args outputDir.get().asFile.toString()
                    args "-s"
                    args namespaceProvider.get().toString()

                    // Add the potential 'metrics.yaml' files at evaluation-time, rather than
                    // configuration-time. Otherwise the Gradle build will fail.
                    inputs.files.filter { it.exists() }.forEach { file ->
                        logger.lifecycle("Glean SDK - generating API from ${file.path}")
                        args file.path
                    }
                }

                // Only show the output if something went wrong.
                ignoreExitValue = true
                standardOutput = System.out
                errorOutput = System.err
                doLast {
                    if (executionResult.get().exitValue != 0) {
                        throw new GradleException("Glean code generation failed.\n\n${standardOutput.toString()}")
                    }
                }
            }

            TaskProvider<Exec> generateGleanMetricsDocs = project.tasks.register("${TASK_NAME_PREFIX}DocsFor${variant.name.capitalize()}", Exec) {
                if (installGleanParser != null) {
                    dependsOn(installGleanParser)
                }
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
                            it.attribute(ArtifactTypeDefinition.ARTIFACT_TYPE_ATTRIBUTE, 'glean-metrics-yaml')
                        }
                    }.files
                }

                // Add local registry files as input to this task.
                // They will be turned into `arg`s later.
                inputs.files(getYamlFiles(project)).optional(true)

                outputs.dir gleanDocsDirectory
                workingDir project.rootDir
                commandLine getPythonCommand(envDir)

                args "-c"
                args runPythonScript
                args isOffline ? "offline" : "online"
                args "glean_parser"
                args parserVersion
                args "translate"
                args "--allow-missing-files"
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

                // Enable expiration by major version, if a major version is provided.
                if (project.ext.has("gleanExpireByVersion")) {
                    args "--expire-by-version=${project.ext.get("gleanExpireByVersion")}"
                }

                doFirst {
                    // Add the potential 'metrics.yaml' files at evaluation-time, rather than
                    // configuration-time. Otherwise the Gradle build will fail.
                    inputs.files.filter { it.exists() }.forEach{ file ->
                        project.logger.lifecycle("Glean SDK - generating docs for ${file.path} in $gleanDocsDirectory")
                        args file.path
                    }
                }

                // Only show the output if something went wrong.
                ignoreExitValue = true
                standardOutput = new ByteArrayOutputStream()
                errorOutput = standardOutput
                doLast {
                    if (executionResult.get().exitValue != 0) {
                        throw new GradleException("Glean documentation generation failed.\n\n${standardOutput.toString()}")
                    }
                }
            }

            // Generate the metrics docs, if requested
            if (project.ext.has("gleanGenerateMarkdownDocs")) {
                generateKotlinAPI.configure {
                    dependsOn(generateGleanMetricsDocs)
                }
            }

            // Use `java`, not `kotlin`: AGP wires generated java dirs into both javac and
            // kotlinc, while `kotlin` is only consumed by AGP's built-in Kotlin support.
            variant.sources.java.addGeneratedSourceDirectory(generateKotlinAPI) { task -> task.outputDir }
        }
    }

    File setupPythonEnvironmentTasks(Project project, String parserVersion) {
        // 1. We use the system Python on the PATH, or the one set by GLEAN_PYTHON.
        // 2. We create a virtual environment in ~/.gradle/glean/pythonenv based on that
        //    Python, so it is shared between multiple projects using Glean.
        // 3. glean_parser is installed using pip from pypi.org. In offline mode we instead
        //    expect the wheels for glean_parser and all its dependencies in
        //    $rootDir/glean-wheels, or GLEAN_PYTHON_WHEELS_DIR. These can be downloaded in
        //    advance easily with `pip download glean_parser`.
        File envDir = new File(
            project.getGradle().gradleUserHomeDir,
            "glean/pythonenv"
        )

        TaskProvider<Exec> createGleanPythonVirtualEnv = project.tasks.register("createGleanPythonVirtualEnv", Exec) {
            description = "Create a Python virtual environment for Glean"

            outputs.dir(envDir)

            String pythonBinary = System.getenv("GLEAN_PYTHON")
            if (!pythonBinary) {
                if (Os.isFamily(Os.FAMILY_WINDOWS)) {
                    pythonBinary = "python"
                } else {
                    pythonBinary = "python3"
                }
            }

            if (isOffline) {
                project.logger.warn("Building in offline mode, therefore, Glean is using a supplied Python at ${pythonBinary}")
                project.logger.warn("The Python binary can be overridden with the GLEAN_PYTHON env var.")
            } else {
                project.logger.info("Glean is using the Python at ${pythonBinary}, overridable with the GLEAN_PYTHON env var.")
            }

            commandLine pythonBinary
            args "-m"
            args "venv"
            args envDir.toString()
        }

        installGleanParser = project.tasks.register("installGleanParser", Exec) {
            description = "Install glean_parser"

            outputs.dir(envDir)

            commandLine getPythonCommand(envDir)
            args "-m"
            args "pip"
            args "install"

            if (isOffline) {
                String pythonPackagesDir = System.getenv("GLEAN_PYTHON_WHEELS_DIR")
                if (!pythonPackagesDir) {
                    pythonPackagesDir = "${project.rootDir}/glean-wheels"
                }

                project.logger.warn("Installing glean_parser from cached Python packages in ${pythonPackagesDir}")
                project.logger.warn("This can be overridden with the GLEAN_PYTHON_WHEELS_DIR env var.")

                args "glean_parser"
                args "--no-index"
                args "-f"
                args pythonPackagesDir
            } else {
                // A git package (a la `git+https://github.com`) is installed as given.
                args(parserVersion.matches("git.+") ? parserVersion : "glean_parser~=${parserVersion}")
            }
        }

        installGleanParser.configure {
            dependsOn(createGleanPythonVirtualEnv)
        }
        project.preBuild.finalizedBy(installGleanParser)

        return envDir
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
                registerTransform(GleanMetricsYamlTransform) {
                    // The type here should be
                    // `com.android.build.gradle.internal.publishing.AndroidArtifacts.ArtifactType.EXPLODED_AAR.getType())`,
                    // but there's no good way to access the including script's classpath from `apply from:`
                    // scripts. See https://stackoverflow.com/a/37060550. The 'android-exploded-aar' string is
                    // very unlikely to change, so it's just hard-coded.
                    from.attribute(ArtifactTypeDefinition.ARTIFACT_TYPE_ATTRIBUTE, "android-exploded-aar")
                    to.attribute(ArtifactTypeDefinition.ARTIFACT_TYPE_ATTRIBUTE, "glean-metrics-yaml")
                }
            }
        }
    }

    def gleanParserVersion(Project project) {
        if (project.ext.has("gleanParserOverride")) {
            println("glean_parser override detected. Using ${project.ext.gleanParserOverride}")
            return project.ext.gleanParserOverride
        } else {
            return GLEAN_PARSER_VERSION
        }
    }

    void apply(Project project) {
        isOffline = project.gradle.startParameter.offline

        project.ext.glean_version = "70.0.0"
        def parserVersion = gleanParserVersion(project)

        // Print the required glean_parser version to the console. This is
        // offline builds, and is mentioned in the documentation for offline
        // builds.
        println("Requires glean_parser ${parserVersion}")

        File envDir
        if (project.ext.has("gleanPythonEnvDir")) {
            envDir = new File(project.ext.gleanPythonEnvDir)
            isOffline = true
        } else {
            envDir = setupPythonEnvironmentTasks(project, parserVersion)
            project.ext.set("gleanPythonEnvDir", envDir)
        }

        setupExtractMetricsFromAARTasks(project)

        project.configurations.all {
            resolutionStrategy.capabilitiesResolution.withCapability("org.mozilla.telemetry:glean-native") {
                def toBeSelected = candidates.find { it.id instanceof ModuleComponentIdentifier && it.id.module.contains('geckoview') }
                if (toBeSelected != null) {
                    select(toBeSelected)
                }
                because 'use GeckoView Glean instead of standalone Glean'
            }
        }

        def isApplication = project.plugins.hasPlugin('com.android.application')
        def androidComponents = project.extensions.getByName('androidComponents')
        androidComponents.onVariants(androidComponents.selector().all(), setupTasks(project, envDir, isApplication, parserVersion))
    }
}

// Put an instance of the plugin in ext so it can be used from the outside
// by Glean's own projects. This is not used by third-parties when using the
// plugin.
ext.glean_plugin = new GleanPlugin()
