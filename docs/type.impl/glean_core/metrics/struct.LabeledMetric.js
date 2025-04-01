(function() {
    var type_impls = Object.fromEntries([["glean_core",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-LabeledMetric%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/glean_core/metrics/labeled.rs.html#77\">source</a><a href=\"#impl-Debug-for-LabeledMetric%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.LabeledMetric.html\" title=\"struct glean_core::metrics::LabeledMetric\">LabeledMetric</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/glean_core/metrics/labeled.rs.html#77\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","glean_core::metrics::labeled::LabeledCounter","glean_core::metrics::labeled::LabeledBoolean","glean_core::metrics::labeled::LabeledString","glean_core::metrics::labeled::LabeledCustomDistribution","glean_core::metrics::labeled::LabeledMemoryDistribution","glean_core::metrics::labeled::LabeledTimingDistribution","glean_core::metrics::labeled::LabeledQuantity"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-LabeledMetric%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/glean_core/metrics/labeled.rs.html#196-321\">source</a><a href=\"#impl-LabeledMetric%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"struct\" href=\"glean_core/metrics/struct.LabeledMetric.html\" title=\"struct glean_core::metrics::LabeledMetric\">LabeledMetric</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"glean_core/trait.AllowLabeled.html\" title=\"trait glean_core::AllowLabeled\">AllowLabeled</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/glean_core/metrics/labeled.rs.html#203-209\">source</a><h4 class=\"code-header\">pub fn <a href=\"glean_core/metrics/struct.LabeledMetric.html#tymethod.new\" class=\"fn\">new</a>(\n    meta: <a class=\"enum\" href=\"glean_core/enum.LabeledMetricData.html\" title=\"enum glean_core::LabeledMetricData\">LabeledMetricData</a>,\n    labels: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/alloc/borrow/enum.Cow.html\" title=\"enum alloc::borrow::Cow\">Cow</a>&lt;'static, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.str.html\">str</a>&gt;&gt;&gt;,\n) -&gt; <a class=\"struct\" href=\"glean_core/metrics/struct.LabeledMetric.html\" title=\"struct glean_core::metrics::LabeledMetric\">LabeledMetric</a>&lt;T&gt;</h4></section></summary><div class=\"docblock\"><p>Creates a new labeled metric from the given metric instance and optional list of labels.</p>\n<p>See <a href=\"glean_core/metrics/struct.LabeledMetric.html#method.get\" title=\"method glean_core::metrics::LabeledMetric::get\"><code>get</code></a> for information on how static or dynamic labels are handled.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get\" class=\"method\"><a class=\"src rightside\" href=\"src/glean_core/metrics/labeled.rs.html#270-302\">source</a><h4 class=\"code-header\">pub fn <a href=\"glean_core/metrics/struct.LabeledMetric.html#tymethod.get\" class=\"fn\">get</a>&lt;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.str.html\">str</a>&gt;&gt;(&amp;self, label: S) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;T&gt;</h4></section></summary><div class=\"docblock\"><p>Gets a specific metric for a given label.</p>\n<p>If a set of acceptable labels were specified in the <code>metrics.yaml</code> file,\nand the given label is not in the set, it will be recorded under the special <code>OTHER_LABEL</code> label.</p>\n<p>If a set of acceptable labels was not specified in the <code>metrics.yaml</code> file,\nonly the first 16 unique labels will be used.\nAfter that, any additional labels will be recorded under the special <code>OTHER_LABEL</code> label.</p>\n<p>Labels must be <code>snake_case</code> and less than 30 characters.\nIf an invalid label is used, the metric will be recorded in the special <code>OTHER_LABEL</code> label.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.test_get_num_recorded_errors\" class=\"method\"><a class=\"src rightside\" href=\"src/glean_core/metrics/labeled.rs.html#315-320\">source</a><h4 class=\"code-header\">pub fn <a href=\"glean_core/metrics/struct.LabeledMetric.html#tymethod.test_get_num_recorded_errors\" class=\"fn\">test_get_num_recorded_errors</a>(&amp;self, error: <a class=\"enum\" href=\"glean_core/enum.ErrorType.html\" title=\"enum glean_core::ErrorType\">ErrorType</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i32.html\">i32</a></h4></section></summary><div class=\"docblock\"><p><strong>Exported for test purposes.</strong></p>\n<p>Gets the number of recorded errors for the given metric and error type.</p>\n<h5 id=\"arguments\"><a class=\"doc-anchor\" href=\"#arguments\">§</a>Arguments</h5>\n<ul>\n<li><code>error</code> - The type of error</li>\n</ul>\n<h5 id=\"returns\"><a class=\"doc-anchor\" href=\"#returns\">§</a>Returns</h5>\n<p>The number of errors reported.</p>\n</div></details></div></details>",0,"glean_core::metrics::labeled::LabeledCounter","glean_core::metrics::labeled::LabeledBoolean","glean_core::metrics::labeled::LabeledString","glean_core::metrics::labeled::LabeledCustomDistribution","glean_core::metrics::labeled::LabeledMemoryDistribution","glean_core::metrics::labeled::LabeledTimingDistribution","glean_core::metrics::labeled::LabeledQuantity"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[7394]}