package mozilla.telemetry.glean

import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.RustError

class BooleanMetricType {
    var handle: Long

    constructor(category: String, name: String) {
        println("New Boolean: $category.$name")

        val e = RustError.ByReference()
        this.handle = LibGleanFFI.INSTANCE.glean_new_boolean_metric(category, name, e)
    }

    /**
     * Set a boolean value.
     *
     * @param value This is a user defined boolean value.
     */
    fun set(value: Boolean) {
        val e = RustError.ByReference()
        LibGleanFFI.INSTANCE.glean_boolean_set(Glean.handle(), this.handle, if (value) { 1 } else { 0 }, e)
    }
}
