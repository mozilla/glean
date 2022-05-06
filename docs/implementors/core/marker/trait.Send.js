(function() {var implementors = {};
implementors["glean"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/struct.Configuration.html\" title=\"struct glean::Configuration\">Configuration</a>","synthetic":true,"types":["glean::configuration::Configuration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/struct.ClientInfoMetrics.html\" title=\"struct glean::ClientInfoMetrics\">ClientInfoMetrics</a>","synthetic":true,"types":["glean::core_metrics::ClientInfoMetrics"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/net/struct.HttpUploader.html\" title=\"struct glean::net::HttpUploader\">HttpUploader</a>","synthetic":true,"types":["glean::net::http_uploader::HttpUploader"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.BooleanMetric.html\" title=\"struct glean::private::BooleanMetric\">BooleanMetric</a>","synthetic":true,"types":["glean::private::boolean::BooleanMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.CounterMetric.html\" title=\"struct glean::private::CounterMetric\">CounterMetric</a>","synthetic":true,"types":["glean::private::counter::CounterMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.CustomDistributionMetric.html\" title=\"struct glean::private::CustomDistributionMetric\">CustomDistributionMetric</a>","synthetic":true,"types":["glean::private::custom_distribution::CustomDistributionMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.DatetimeMetric.html\" title=\"struct glean::private::DatetimeMetric\">DatetimeMetric</a>","synthetic":true,"types":["glean::private::datetime::DatetimeMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.DenominatorMetric.html\" title=\"struct glean::private::DenominatorMetric\">DenominatorMetric</a>","synthetic":true,"types":["glean::private::denominator::DenominatorMetric"]},{"text":"impl&lt;K&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.EventMetric.html\" title=\"struct glean::private::EventMetric\">EventMetric</a>&lt;K&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["glean::private::event::EventMetric"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.LabeledMetric.html\" title=\"struct glean::private::LabeledMetric\">LabeledMetric</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Sealed&gt;::Inner: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["glean::private::labeled::LabeledMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.MemoryDistributionMetric.html\" title=\"struct glean::private::MemoryDistributionMetric\">MemoryDistributionMetric</a>","synthetic":true,"types":["glean::private::memory_distribution::MemoryDistributionMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.NumeratorMetric.html\" title=\"struct glean::private::NumeratorMetric\">NumeratorMetric</a>","synthetic":true,"types":["glean::private::numerator::NumeratorMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.PingType.html\" title=\"struct glean::private::PingType\">PingType</a>","synthetic":true,"types":["glean::private::ping::PingType"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.QuantityMetric.html\" title=\"struct glean::private::QuantityMetric\">QuantityMetric</a>","synthetic":true,"types":["glean::private::quantity::QuantityMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.RateMetric.html\" title=\"struct glean::private::RateMetric\">RateMetric</a>","synthetic":true,"types":["glean::private::rate::RateMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.RecordedExperimentData.html\" title=\"struct glean::private::RecordedExperimentData\">RecordedExperimentData</a>","synthetic":true,"types":["glean::private::recorded_experiment_data::RecordedExperimentData"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.StringMetric.html\" title=\"struct glean::private::StringMetric\">StringMetric</a>","synthetic":true,"types":["glean::private::string::StringMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.StringListMetric.html\" title=\"struct glean::private::StringListMetric\">StringListMetric</a>","synthetic":true,"types":["glean::private::string_list::StringListMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.TimespanMetric.html\" title=\"struct glean::private::TimespanMetric\">TimespanMetric</a>","synthetic":true,"types":["glean::private::timespan::TimespanMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.TimingDistributionMetric.html\" title=\"struct glean::private::TimingDistributionMetric\">TimingDistributionMetric</a>","synthetic":true,"types":["glean::private::timing_distribution::TimingDistributionMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.UrlMetric.html\" title=\"struct glean::private::UrlMetric\">UrlMetric</a>","synthetic":true,"types":["glean::private::url::UrlMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean/private/struct.UuidMetric.html\" title=\"struct glean::private::UuidMetric\">UuidMetric</a>","synthetic":true,"types":["glean::private::uuid::UuidMetric"]}];
implementors["glean_core"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_core/enum.Lifetime.html\" title=\"enum glean_core::Lifetime\">Lifetime</a>","synthetic":true,"types":["glean_core::common_metric_data::Lifetime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/struct.CommonMetricData.html\" title=\"struct glean_core::CommonMetricData\">CommonMetricData</a>","synthetic":true,"types":["glean_core::common_metric_data::CommonMetricData"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_core/enum.ErrorKind.html\" title=\"enum glean_core::ErrorKind\">ErrorKind</a>","synthetic":true,"types":["glean_core::error::ErrorKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/struct.Error.html\" title=\"struct glean_core::Error\">Error</a>","synthetic":true,"types":["glean_core::error::Error"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_core/enum.ErrorType.html\" title=\"enum glean_core::ErrorType\">ErrorType</a>","synthetic":true,"types":["glean_core::error_recording::ErrorType"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.RecordedEvent.html\" title=\"struct glean_core::metrics::RecordedEvent\">RecordedEvent</a>","synthetic":true,"types":["glean_core::event_database::RecordedEvent"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_core/metrics/enum.HistogramType.html\" title=\"enum glean_core::metrics::HistogramType\">HistogramType</a>","synthetic":true,"types":["glean_core::histogram::HistogramType"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.BooleanMetric.html\" title=\"struct glean_core::metrics::BooleanMetric\">BooleanMetric</a>","synthetic":true,"types":["glean_core::metrics::boolean::BooleanMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.CounterMetric.html\" title=\"struct glean_core::metrics::CounterMetric\">CounterMetric</a>","synthetic":true,"types":["glean_core::metrics::counter::CounterMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.CustomDistributionMetric.html\" title=\"struct glean_core::metrics::CustomDistributionMetric\">CustomDistributionMetric</a>","synthetic":true,"types":["glean_core::metrics::custom_distribution::CustomDistributionMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.DatetimeMetric.html\" title=\"struct glean_core::metrics::DatetimeMetric\">DatetimeMetric</a>","synthetic":true,"types":["glean_core::metrics::datetime::DatetimeMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.DenominatorMetric.html\" title=\"struct glean_core::metrics::DenominatorMetric\">DenominatorMetric</a>","synthetic":true,"types":["glean_core::metrics::denominator::DenominatorMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.EventMetric.html\" title=\"struct glean_core::metrics::EventMetric\">EventMetric</a>","synthetic":true,"types":["glean_core::metrics::event::EventMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.JweMetric.html\" title=\"struct glean_core::metrics::JweMetric\">JweMetric</a>","synthetic":true,"types":["glean_core::metrics::jwe::JweMetric"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.LabeledMetric.html\" title=\"struct glean_core::metrics::LabeledMetric\">LabeledMetric</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["glean_core::metrics::labeled::LabeledMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.MemoryDistributionMetric.html\" title=\"struct glean_core::metrics::MemoryDistributionMetric\">MemoryDistributionMetric</a>","synthetic":true,"types":["glean_core::metrics::memory_distribution::MemoryDistributionMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_core/metrics/enum.MemoryUnit.html\" title=\"enum glean_core::metrics::MemoryUnit\">MemoryUnit</a>","synthetic":true,"types":["glean_core::metrics::memory_unit::MemoryUnit"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.PingType.html\" title=\"struct glean_core::metrics::PingType\">PingType</a>","synthetic":true,"types":["glean_core::metrics::ping::PingType"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.QuantityMetric.html\" title=\"struct glean_core::metrics::QuantityMetric\">QuantityMetric</a>","synthetic":true,"types":["glean_core::metrics::quantity::QuantityMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.RateMetric.html\" title=\"struct glean_core::metrics::RateMetric\">RateMetric</a>","synthetic":true,"types":["glean_core::metrics::rate::RateMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.StringMetric.html\" title=\"struct glean_core::metrics::StringMetric\">StringMetric</a>","synthetic":true,"types":["glean_core::metrics::string::StringMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.StringListMetric.html\" title=\"struct glean_core::metrics::StringListMetric\">StringListMetric</a>","synthetic":true,"types":["glean_core::metrics::string_list::StringListMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_core/metrics/enum.TimeUnit.html\" title=\"enum glean_core::metrics::TimeUnit\">TimeUnit</a>","synthetic":true,"types":["glean_core::metrics::time_unit::TimeUnit"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.TimespanMetric.html\" title=\"struct glean_core::metrics::TimespanMetric\">TimespanMetric</a>","synthetic":true,"types":["glean_core::metrics::timespan::TimespanMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.TimingDistributionMetric.html\" title=\"struct glean_core::metrics::TimingDistributionMetric\">TimingDistributionMetric</a>","synthetic":true,"types":["glean_core::metrics::timing_distribution::TimingDistributionMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.UrlMetric.html\" title=\"struct glean_core::metrics::UrlMetric\">UrlMetric</a>","synthetic":true,"types":["glean_core::metrics::url::UrlMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.UuidMetric.html\" title=\"struct glean_core::metrics::UuidMetric\">UuidMetric</a>","synthetic":true,"types":["glean_core::metrics::uuid::UuidMetric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/metrics/struct.DistributionData.html\" title=\"struct glean_core::metrics::DistributionData\">DistributionData</a>","synthetic":true,"types":["glean_core::metrics::DistributionData"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_core/metrics/enum.Metric.html\" title=\"enum glean_core::metrics::Metric\">Metric</a>","synthetic":true,"types":["glean_core::metrics::Metric"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/ping/struct.Ping.html\" title=\"struct glean_core::ping::Ping\">Ping</a>&lt;'a&gt;","synthetic":true,"types":["glean_core::ping::Ping"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/ping/struct.PingMaker.html\" title=\"struct glean_core::ping::PingMaker\">PingMaker</a>","synthetic":true,"types":["glean_core::ping::PingMaker"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/storage/struct.StorageManager.html\" title=\"struct glean_core::storage::StorageManager\">StorageManager</a>","synthetic":true,"types":["glean_core::storage::StorageManager"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_core/traits/enum.NoExtraKeys.html\" title=\"enum glean_core::traits::NoExtraKeys\">NoExtraKeys</a>","synthetic":true,"types":["glean_core::traits::event::NoExtraKeys"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_core/traits/enum.EventRecordingError.html\" title=\"enum glean_core::traits::EventRecordingError\">EventRecordingError</a>","synthetic":true,"types":["glean_core::traits::event::EventRecordingError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/upload/struct.PingRequest.html\" title=\"struct glean_core::upload::PingRequest\">PingRequest</a>","synthetic":true,"types":["glean_core::upload::request::PingRequest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_core/upload/enum.UploadResult.html\" title=\"enum glean_core::upload::UploadResult\">UploadResult</a>","synthetic":true,"types":["glean_core::upload::result::UploadResult"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_core/upload/enum.PingUploadTask.html\" title=\"enum glean_core::upload::PingUploadTask\">PingUploadTask</a>","synthetic":true,"types":["glean_core::upload::PingUploadTask"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/upload/struct.PingUploadManager.html\" title=\"struct glean_core::upload::PingUploadManager\">PingUploadManager</a>","synthetic":true,"types":["glean_core::upload::PingUploadManager"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/struct.Configuration.html\" title=\"struct glean_core::Configuration\">Configuration</a>","synthetic":true,"types":["glean_core::Configuration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_core/struct.Glean.html\" title=\"struct glean_core::Glean\">Glean</a>","synthetic":true,"types":["glean_core::Glean"]}];
implementors["glean_ffi"] = [{"text":"impl !<a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_ffi/byte_buffer/struct.ByteBuffer.html\" title=\"struct glean_ffi::byte_buffer::ByteBuffer\">ByteBuffer</a>","synthetic":true,"types":["glean_ffi::byte_buffer::ByteBuffer"]},{"text":"impl !<a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"glean_ffi/upload/enum.FfiPingUploadTask.html\" title=\"enum glean_ffi::upload::FfiPingUploadTask\">FfiPingUploadTask</a>","synthetic":true,"types":["glean_ffi::upload::FfiPingUploadTask"]},{"text":"impl&lt;'a&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"glean_ffi/struct.FfiConfiguration.html\" title=\"struct glean_ffi::FfiConfiguration\">FfiConfiguration</a>&lt;'a&gt;","synthetic":true,"types":["glean_ffi::FfiConfiguration"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()