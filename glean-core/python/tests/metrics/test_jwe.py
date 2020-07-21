# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# flake8: noqa E501

from glean import metrics
from glean.metrics import Lifetime
from glean import testing

header = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ"
key = "OKOawDo13gRp2ojaHV7LFpZcgV7T6DVZKTyKOMTYUmKoTCVJRgckCL9kiMT03JGeipsEdY3mx_etLbbWSrFr05kLzcSr4qKAq7YN7e9jwQRb23nfa6c9d-StnImGyFDbSv04uVuxIp5Zms1gNxKKK2Da14B8S4rzVRltdYwam_lDp5XnZAYpQdb76FdIKLaVmqgfwX7XWRxv2322i-vDxRfqNzo_tETKzpVLzfiwQyeyPGLBIO56YJ7eObdv0je81860ppamavo35UgoRdbYaBcoh9QcfylQr66oc6vFWXRcZ_ZT2LawVCWTIy3brGPi6UklfCpIMfIjf7iGdXKHzg"
init_vector = "48V1_ALb6US04U3b"
cipher_text = "5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A"
auth_tag = "XFBoMYUZodetZdvTiFvSkQ"
jwe = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ.OKOawDo13gRp2ojaHV7LFpZcgV7T6DVZKTyKOMTYUmKoTCVJRgckCL9kiMT03JGeipsEdY3mx_etLbbWSrFr05kLzcSr4qKAq7YN7e9jwQRb23nfa6c9d-StnImGyFDbSv04uVuxIp5Zms1gNxKKK2Da14B8S4rzVRltdYwam_lDp5XnZAYpQdb76FdIKLaVmqgfwX7XWRxv2322i-vDxRfqNzo_tETKzpVLzfiwQyeyPGLBIO56YJ7eObdv0je81860ppamavo35UgoRdbYaBcoh9QcfylQr66oc6vFWXRcZ_ZT2LawVCWTIy3brGPi6UklfCpIMfIjf7iGdXKHzg.48V1_ALb6US04U3b.5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A.XFBoMYUZodetZdvTiFvSkQ"
minimum_jwe = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ...5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A."


def test_the_api_saves_to_its_storage_engine():
    jwe_metric = metrics.JweMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="jwe_metric",
        send_in_pings=["store1"],
    )

    jwe_metric.set(header, key, init_vector, cipher_text, auth_tag)

    assert jwe_metric.test_has_value()
    assert jwe == jwe_metric.test_get_compact_representation()

    jwe_metric.set(header, "", "", cipher_text, "")

    assert jwe_metric.test_has_value()
    assert minimum_jwe == jwe_metric.test_get_compact_representation()


def test_disabled_jwes_must_not_record_data():
    jwe_metric = metrics.JweMetricType(
        disabled=True,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="jwe_metric",
        send_in_pings=["store1"],
    )

    jwe_metric.set(header, key, init_vector, cipher_text, auth_tag)

    assert not jwe_metric.test_has_value()


def test_jwe_get_value_returns_correct_jwe_data_representation():
    jwe_metric = metrics.JweMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="jwe_metric",
        send_in_pings=["store1"],
    )

    jwe_metric.set(header, key, init_vector, cipher_text, auth_tag)

    data = jwe_metric.test_get_value()
    assert data.header == header
    assert data.key == key
    assert data.init_vector == init_vector
    assert data.cipher_text == cipher_text
    assert data.auth_tag == auth_tag


def test_the_api_saves_to_secondary_pings():
    jwe_metric = metrics.JweMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="jwe_metric",
        send_in_pings=["store1", "store2"],
    )

    jwe_metric.set(header, key, init_vector, cipher_text, auth_tag)

    assert jwe_metric.test_has_value("store2")
    assert jwe == jwe_metric.test_get_compact_representation("store2")

    jwe_metric.set(header, "", "", cipher_text, "")

    assert jwe_metric.test_has_value("store2")
    assert minimum_jwe == jwe_metric.test_get_compact_representation("store2")


def test_setting_invalid_values_record_errors():
    jwe_metric = metrics.JweMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="jwe_metric",
        send_in_pings=["store1", "store2"],
    )

    jwe_metric.set("X" * 1025, key, init_vector, cipher_text, auth_tag)
    assert 1 == jwe_metric.test_get_num_recorded_errors(
        testing.ErrorType.INVALID_OVERFLOW
    )

    jwe_metric.set_with_compact_representation("")
    assert 1 == jwe_metric.test_get_num_recorded_errors(testing.ErrorType.INVALID_VALUE)
