// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Security;
using Microsoft.Build.Framework;
using Microsoft.Build.Utilities;

namespace GleanTasks
{
    public class GleanParser : ToolTask
    {
        // The name of the directory in which the Python virtual
        // environment must be created.
        private const string DefaultVirtualEnvDir = ".venv";

        // The glean_parser pypi package version 
        private const string GleanParserVersion = "1.29.0";

        // This script runs a given Python module as a "main" module, like
        // `python -m module`. However, it first checks that the installed
        // package is at the desired version, and if not, upgrades it using `pip`.
        //
        // ** IMPORTANT**
        // Keep this script in sync with the one in `glean-gradle-plugin/GleanGradlePlugin.groovy`.
        private const string RunPythonScript = @"
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
try:
    subprocess.check_call([
        sys.executable,
        '-m',
        module_name
    ] + sys.argv[3:])
except:
    # We don't need to show a traceback in this helper script.
    # Only the output of the subprocess is interesting.
    sys.exit(1)
";

        /// <summary>
        /// The Glean registry files to process.
        /// </summary>
        [Required]
        public ITaskItem[] RegistryFiles { get; set; }

        [Required]
        public string OutputPath { get; set; }

        /// <summary>
        /// The namespace in which to generate the Glean metrics.
        /// </summary>
        [Required]
        public string Namespace { get; set; }

        /// <summary>
        /// This *MUST* only be used by the Glean SDK.
        ///
        /// Whether or not to allow using Glean reserved names. Defaults to
        /// `false`.
        /// </summary>
        public bool AllowReserved { get; set; } = false;

        private bool IsWindows() => RuntimeInformation.IsOSPlatform(OSPlatform.Windows);

        // Make sure that we can see the output from the glean_parser when
        // building.
        protected override MessageImportance StandardOutputLoggingImportance
        {
            get { return MessageImportance.Normal; }
        }

        protected override string ToolName
        { 
            get {
                try
                {
                    // If the "GLEAN_PYTHON" environment variable is set, use it and don't
                    // look in the system PATH for a python interpreter.
                    string gleanFromEnv = Environment.GetEnvironmentVariable("GLEAN_PYTHON");
                    if (gleanFromEnv != null)
                    {
                        return gleanFromEnv;
                    }
                }
                catch (SecurityException)
                {
                    Log.LogMessage(
                        MessageImportance.Low, "Failed to look for the GLEAN_PYTHON environment variable");
                }

                // No "GLEAN_PYTHON" is available, look for the Python interpreter in the PATH.
                if (IsWindows())
                {
                    return "python.exe";
                }

                // We don't need the '.exe' suffix outside of Windows. The
                // canonical name of the file for Python3 is... "python3".
                return "python3";
            }
        }
        
        /// <summary>
        /// Called to get the full path of the tool.
        /// 
        /// When returning `ToolExe`, this will look for the tool in the PATH.
        /// If `ToolPath` is set, then the value of this function is ignored.
        /// </summary>
        /// <returns>A string representing the path to the tool</returns>
        protected override string GenerateFullPathToTool()
        { 
            return ToolExe;
        }

        /// <summary>
        /// Get the path to the Virtual Environment directory containing the
        /// Python interpreter.
        /// </summary>
        /// <param name="venvDir">The path of the virtual environment directory</param>
        /// <returns>The path to the "Scripts" or "bin" dir in the virtual environment</returns>
        private string GetVirtualEnvironmentPath(string venvDir)
        {
            // "Scripts" is only there on windows. On Linux/Mac is "bin"
            if (IsWindows())
            {
                return Path.Combine(venvDir, "Scripts");
            }

            return Path.Combine(venvDir, "bin");
        }

        /// <summary>
        /// Checks if a Python virtual environment is available.
        /// </summary>
        /// <param name="venvDir">The python virtual environment directory</param>
        /// <returns>True if the virtual environment exists, false otherwise</returns>
        private bool CheckVirtualEnvironmentExists(string venvDir)
        {
            string venvPython = Path.Combine(GetVirtualEnvironmentPath(venvDir), ToolName);

            if (Directory.Exists(DefaultVirtualEnvDir)
                && File.Exists(venvPython))
            {
                return true;
            }

            return false;
        }

        /// <summary>
        /// Uses a Python interpreter to create a Python3 virtual environment.
        /// </summary>
        /// <param name="venvDir">The directory in which to create the virtual environemnt</param>
        /// <returns>True if the environment was correctly created, false otherwise</returns>
        private bool CreateVirtualEnvironment(string venvDir)
        {
            Log.LogMessage(MessageImportance.High, "Creating a Glean SDK virtual environment at: " + venvDir);
            
            CommandLineBuilder venvCommand = new CommandLineBuilder();
            venvCommand.AppendSwitch("-m venv " + venvDir);

            if (ExecuteTool(GenerateFullPathToTool(), string.Empty, venvCommand.ToString()) != 0)
            {
                // We failed to create the virtual environment. Bail out.
                return false;
            }

            // Install bdist_wheel.
            string pipTool = Path.Combine(GetVirtualEnvironmentPath(venvDir), IsWindows() ? "pip3.exe" : "pip3");

            CommandLineBuilder pipCommand = new CommandLineBuilder();
            pipCommand.AppendSwitch("install");
            pipCommand.AppendSwitch("wheel");

            return ExecuteTool(pipTool, string.Empty, pipCommand.ToString()) == 0;
        }

        private bool Setup()
        {
            if (CheckVirtualEnvironmentExists(DefaultVirtualEnvDir))
            {
                // The virtual environment was already created. Try to use it!
                Log.LogMessage(MessageImportance.High, "Using Glean SDK virtual environment at: " + DefaultVirtualEnvDir);
            } else if (!CreateVirtualEnvironment(DefaultVirtualEnvDir))
            {
                // Attempt to create a new virtual environemnt. If we fail there's
                // nothing we could possibly do to move forward.
                Log.LogError("Failed to create a Glean SDK virtual environment at: " + DefaultVirtualEnvDir);
                return false;
            }

            // Set the python instance to use from now on! Once we set this to a path,
            // this will be the one used by `Execute`.
            ToolPath = GetVirtualEnvironmentPath(DefaultVirtualEnvDir);

            return true;
        }

        /// <summary>
        /// Invoked by ToolTask to generate the command to execute.
        /// </summary>
        /// <returns>A string representing the command to execute</returns>
        protected override string GenerateCommandLineCommands()
        {
            CommandLineBuilder builder = new CommandLineBuilder();
            builder.AppendSwitch("-c");
            builder.AppendTextUnquoted(" \"" + RunPythonScript + "\" ");
            builder.AppendSwitch("glean_parser");
            builder.AppendSwitch(GleanParserVersion);
            builder.AppendSwitch("translate");
            builder.AppendSwitch("-f \"csharp\"");
            builder.AppendSwitch("-o");
            builder.AppendFileNameIfNotNull(OutputPath);
            builder.AppendSwitch("-s \"glean_namespace=Mozilla.Glean\"");
            builder.AppendSwitch($"-s \"namespace={Namespace}\"");
            if (AllowReserved)
            {
                builder.AppendSwitch("--allow-reserved");
            }

            foreach (ITaskItem file in RegistryFiles)
            {
                builder.AppendFileNameIfNotNull(Path.GetFullPath(file.ItemSpec));
            }

            Log.LogMessage(MessageImportance.Low, "GleanParser.GenerateCommandLineCommands command: " + builder.ToString());

            return builder.ToString();
        }

        public override bool Execute()
        {
            if (!Setup())
            {
                Log.LogError("Failed to setup the Glean SDK build environment");
                return false;
            }

            bool result = base.Execute();
            if (!result)
            {
                return false;
            }

            return true;
        }
    }
}
