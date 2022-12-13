/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.samples.gleancore

import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import mozilla.telemetry.glean.Glean
import org.mozilla.samples.gleancore.GleanMetrics.BrowserEngagement
import org.mozilla.samples.gleancore.GleanMetrics.Pings
import org.mozilla.samples.gleancore.GleanMetrics.Test
import org.mozilla.samples.gleancore.databinding.ActivityMainBinding

open class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)

        setContentView(binding.root)

        Test.isStarted.set(true)

        // Generate an event when user clicks on the button.
        binding.buttonGenerateData.setOnClickListener {
            // These first two actions, adding to the string list and incrementing the counter are
            // tied to a user lifetime metric which is persistent from launch to launch.

            // Adds the EditText's text content as a new string in the string list metric from the
            // metrics.yaml file.
            Test.stringList.add(binding.etStringListInput.text.toString())
            // Clear current text to help indicate something happened
            binding.etStringListInput.setText("")

            // Increments the test_counter metric from the metrics.yaml file.
            Test.counter.add()

            // This is referencing the event ping named 'click' from the metrics.yaml file. In
            // order to illustrate adding extra information to the event, it is also adding to the
            // 'extras' field a dictionary of values.  Note that the dictionary keys must be
            // declared in the metrics.yaml file under the 'extra_keys' section of an event metric.
            BrowserEngagement.click.record(BrowserEngagement.ClickExtra(key1 = "extra_value_1", key2 = "extra_value_2"))

            // An event without any extra keys
            BrowserEngagement.eventNoKeys.record()

            val text = "Data generated"
            val duration = Toast.LENGTH_SHORT
            val toast = Toast.makeText(applicationContext, text, duration)
            toast.show()
        }

        binding.buttonSubmit.setOnClickListener {
            Pings.sample.submit()
            val text = "Sample ping submitted"
            val duration = Toast.LENGTH_SHORT
            val toast = Toast.makeText(applicationContext, text, duration)
            toast.show()
        }

        binding.uploadSwitch.setOnCheckedChangeListener { _, isChecked ->
            if (isChecked) {
                binding.gleanEnabledText.setText("Glean is enabled")
                Glean.setUploadEnabled(true)
            } else {
                binding.gleanEnabledText.setText("Glean is disabled")
                Glean.setUploadEnabled(false)
            }
        }

        Test.timespan.stop()
    }
}
