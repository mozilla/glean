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
import org.gradle.api.artifacts.DependencyResolveDetails
import org.gradle.api.artifacts.component.ModuleComponentSelector
import org.gradle.api.artifacts.ResolvedArtifact
import org.gradle.api.artifacts.ResolveException

// The suppression "GrPackage" is needed below since Android Studio wants this file to have
// a package name, but adding one causes the build to fail with:
//    "'.../GleanGradlePlugin.groovy' should not contain a package statement"
// due to how this file is included directly in the local build.

// This is the plugin that third-party packages use.  It works as follows:
//
// - It navigates the dependency tree to find the version of Glean that is being used.
// - Downloads the actual Glean plugin in GleanGradlePluginCore.groovy from the correct
//   tag on Github, caching it so this network request doesn't happen every time.
// - Runs that plugin.
@SuppressWarnings("GrPackage")
class GleanPlugin implements Plugin<Project> {

    // Get the version of Glean currently being used in this project
    String getGleanSdkVersion(Project project) {
        def gleanSdkVersion = "Not Found"

        project.getConfigurations().all { config ->
            if (!config.name.startsWith(":app:")) {
                return
            }

            def resolved
            try {
                resolved = config.resolvedConfiguration
            } catch (IllegalStateException e) {
                return
            }

            def artifacts
            try {
                artifacts = resolved.getResolvedArtifacts()
            } catch (ResolveException e) {
                return
            }

            println(config)
            artifacts.forEach { ResolvedArtifact artifact ->
                def versionId = artifact.moduleVersion.id
                if (!versionId.group.startsWith("org.mozilla.telemetry")) {
                    return
                }

                if (versionId.name.startsWith("glean")) {
                    gleanSdkVersion = versionId.version
                }
            }
        }

        if (gleanSdkVersion == "Not Found") {
            return "master"
        }

        if (gleanSdkVersion.endsWith("-SNAPSHOT")) {
            return "master"
        }

        def parts = gleanSdkVersion.tokenize(".")
        if (parts.size() > 0 && parts[0].toInteger() <= 19) {
            return "master"
        }

        return gleanSdkVersion
    }

    void apply(Project project) {
        def gleanSdkVersion = getGleanSdkVersion(project)

        File gleanDataDir = new File(project.getGradle().gradleUserHomeDir, "glean")
        if (!gleanDataDir.exists()) {
            gleanDataDir.mkdir()
        }

        File localPluginCache = new File(
            gleanDataDir,
            "gradle-plugin-${gleanSdkVersion}.groovy"
        )

        if (!localPluginCache.exists()) {
            // def url = "https://raw.githubusercontent.com/mozilla/glean/${gleanSdkVersion}/gradle-plugin/GleanGradlePluginCore.groovy"

            // Hardcode the URL until it's in its new place...
            def url = "https://raw.githubusercontent.com/mozilla/glean/master/gradle-plugin/src/main/groovy/mozilla/telemetry/glean-gradle-plugin/GleanGradlePlugin.groovy"

            localPluginCache.withOutputStream { out -> out << new URL(url).openStream() } 
        }

        project.apply from: localPluginCache.path
        project.ext.glean_plugin.apply(project)
    }
}
