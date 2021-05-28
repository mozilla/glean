# Adding Glean to your C# project

TODO. To be implemented in [bug 1643568](https://bugzilla.mozilla.org/show_bug.cgi?id=1643568).

## Setting up metrics and pings code generation

A new build target needs to be added to the project `csproj` file in order to generate the metrics and pings APIs from the registry files (e.g. `metrics.yaml`, `pings.yaml`).

```xml
<Project>
  <!-- ... other directives ... -->

  <Target Name="GleanIntegration" BeforeTargets="CoreCompile">
    <ItemGroup>
      <!--
        Note that the two files are not required: Glean will work just fine
        with just the 'metrics.yaml'. A 'pings.yaml' is only required if custom
        pings are defined.
        Please also note that more than one metrics file can be added.
      -->
      <GleanRegistryFiles Include="metrics.yaml" />
      <GleanRegistryFiles Include="pings.yaml" />
    </ItemGroup>
    <!-- This is what actually runs the parser. -->
    <GleanParser RegistryFiles="@(GleanRegistryFiles)" OutputPath="$(IntermediateOutputPath)Glean" Namespace="csharp.GleanMetrics" />

    <!--
      And this adds the generated files to the project, so that they can be found by
      the compiler and Intellisense.
    -->
    <ItemGroup>
      <Compile Include="$(IntermediateOutputPath)Glean\**\*.cs" />
    </ItemGroup>
  </Target>
</Project>
```

This is using the Python 3 interpreter found in `PATH` under the hood. The `GLEAN_PYTHON` environment variable can be used to provide the location of the Python 3 interpreter.
