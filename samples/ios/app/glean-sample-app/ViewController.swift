/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Glean
import GleanSampleFramework
import UIKit

typealias BrowserEngagement = GleanMetrics.BrowserEngagement

class ViewController: UIViewController {
    @IBOutlet var dataInput: UITextField!
    @IBOutlet var recordButton: UIButton!
    @IBOutlet var sendButton: UIButton!
    @IBOutlet var enabledLabel: UILabel!
    @IBOutlet var enableSwitch: UISwitch!

    override func viewDidLoad() {
        super.viewDidLoad()

        enableSwitch.setOn(Glean.shared.getUploadEnabled(), animated: false)

        Test.isStarted.set(true)

        Test.timespan.stop()
    }

    @IBAction func recordButtonTapped(_: Any) {
        // These first two actions, adding to the string list and incrementing the counter are
        // tied to a user lifetime metric which is persistent from launch to launch.

        // Adds the EditText's text content as a new string in the string list metric from the
        // metrics.yaml file.
        if let text = dataInput.text {
            Test.stringList.add(text)
            // Clear current text to help indicate something happened
            dataInput.text = ""
        }

        // Increments the test_counter metric from the metrics.yaml file.
        Test.counter.add()

        // Increment the custom counter that goes into the sample ping
        Custom.counter.add()

        // This is referencing the event ping named 'click' from the metrics.yaml file. In
        // order to illustrate adding extra information to the event, it is also adding to the
        // 'extras' field a dictionary of values.  Note that the dictionary keys must be
        // declared in the metrics.yaml file under the 'extra_keys' section of an event metric.
        BrowserEngagement.click.record(extra: [
            .key1: "extra_value_1",
            .key2: "extra_value_2"
        ])
    }

    @IBAction func sendButtonTapped(_: Any) {
        // Increment the custom counter that goes into the sample ping
        Custom.counter.add()
        Pings.shared.sample.submit()

        // Activate the glean-sample-framework library function that triggers Glean metric
        // recording
        GleanSampleFramework.doTheThing(withIntensity: 9002)
    }

    @IBAction func enableToggled(_: Any) {
        if enableSwitch.isOn {
            Glean.shared.setUploadEnabled(true)
            enabledLabel.text = "Glean is enabled"
        } else {
            Glean.shared.setUploadEnabled(false)
            enabledLabel.text = "Glean is disabled"
        }
    }
}
