/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.samples.gleancore

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import kotlinx.android.synthetic.main.activity_main.*
import mozilla.telemetry.glean.Glean
import org.mozilla.samples.gleancore.GleanMetrics.Test
import org.mozilla.samples.gleancore.GleanMetrics.BrowserEngagement

open class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        Test.isStarted.set(true)

        // Generate an event when user clicks on the button.
        buttonGenerateData.setOnClickListener {
            // These first two actions, adding to the string list and incrementing the counter are
            // tied to a user lifetime metric which is persistent from launch to launch.

            // Adds the EditText's text content as a new string in the string list metric from the
            // metrics.yaml file.
            Test.stringList.add(etStringListInput.text.toString())
            // Clear current text to help indicate something happened
            etStringListInput.setText("")

            // Increments the test_counter metric from the metrics.yaml file.
            Test.counter.add()

            // This is referencing the event ping named 'click' from the metrics.yaml file. In
            // order to illustrate adding extra information to the event, it is also adding to the
            // 'extras' field a dictionary of values.  Note that the dictionary keys must be
            // declared in the metrics.yaml file under the 'extra_keys' section of an event metric.
            BrowserEngagement.click.record(
                    mapOf(
                        BrowserEngagement.clickKeys.key1 to "extra_value_1",
                        BrowserEngagement.clickKeys.key2 to "extra_value_2"
                    )
            )
        }

        uploadSwitch.setOnCheckedChangeListener { _, isChecked ->
            if (isChecked) {
                gleanEnabledText.setText("Glean is enabled")
                Glean.setUploadEnabled(true)
            } else {
                gleanEnabledText.setText("Glean is disabled")
                Glean.setUploadEnabled(false)
            }
        }

        Test.timespan.stop()
    }
}
