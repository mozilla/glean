/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Glean
import UIKit

typealias BrowserEngagement = GleanMetrics.BrowserEngagement
typealias Party = GleanMetrics.Party

class ViewController: UIViewController {
    let telemetryPrefKey = "GleanUploadEnabled"

    @IBOutlet var dataInput: UITextField!
    @IBOutlet var recordButton: UIButton!
    @IBOutlet var sendButton: UIButton!
    @IBOutlet var enabledLabel: UILabel!
    @IBOutlet var enableSwitch: UISwitch!

    override func viewDidLoad() {
        super.viewDidLoad()

        // Set the state of the upload enabled toggle based on the value in UserDefaults
        if let uploadEnabled = UserDefaults.standard.object(forKey: telemetryPrefKey) as? Bool {
            // There was a value stored, so use it to set the toggle state
            enableSwitch.setOn(uploadEnabled, animated: false)
        } else {
            // There wasn't a value stored, so set the default of `true` for the toggle, and
            // store the preference in UserDefaults
            enableSwitch.setOn(true, animated: false)
            UserDefaults.standard.set(true, forKey: telemetryPrefKey)
        }

        // Set the correct text for the label
        enabledLabel.text = "Glean is \(enableSwitch.isOn ? "enabled" : "disabled")"

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

        // Record an object
        var balloons: Party.BalloonsObject = []
        var labels: Party.BalloonsObjectItemLabels = []
        labels.append("round")
        balloons.append(Party.BalloonsObjectItem(colour: "red", diameter: 5, labels: labels))
        balloons.append(Party.BalloonsObjectItem(colour: "green"))
        Party.balloons.set(balloons)

        var animals: Party.AnimalsObject = []
        animals.append("Dog")
        animals.append("Cat")
        Party.animals.set(animals)

        var ch: Party.ChooserObject = []
        var f = Party.ChooserObjectItem(key: "fortytwo", value: .number(42))
        ch.append(f)
        f = Party.ChooserObjectItem(key: "to-be", value: .boolean(false))
        ch.append(f)
        Party.chooser.set(ch)

        let tlObj = Party.ToplevelObjectObject(key1: "test", anotherValue: 3, subArray: [1, 2, 3])
        Party.toplevelObject.set(tlObj)

        // This is referencing the event ping named 'click' from the metrics.yaml file. In
        // order to illustrate adding extra information to the event, it is also adding to the
        // 'extras' field a dictionary of values.  Note that the dictionary keys must be
        // declared in the metrics.yaml file under the 'extra_keys' section of an event metric.
        BrowserEngagement.click.record(BrowserEngagement.ClickExtra(key1: "extra_value_1", key2: "extra_value_2"))

        // An event without any extra keys
        BrowserEngagement.eventNoKeys.record()
    }

    @IBAction func sendButtonTapped(_: Any) {
        // Increment the custom counter that goes into the sample ping
        Custom.counter.add()
        Pings.shared.sample.submit(reason: .buttonTap)
    }

    @IBAction func enableToggled(_: Any) {
        Glean.shared.setCollectionEnabled(enableSwitch.isOn)
        UserDefaults.standard.set(enableSwitch.isOn, forKey: telemetryPrefKey)
        enabledLabel.text = "Glean is \(enableSwitch.isOn ? "enabled" : "disabled")"
    }
}
