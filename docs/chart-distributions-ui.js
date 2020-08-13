// Transformation function we may want to apply to each value before plotting
//
// The current use cases are memory distributions and timing distribution,
// which may receive the values in a given unit, but transform them to a base one upon recording
let TRANSFORMATION

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!! Memory distribution specific !!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

function memoryUnitToByte(unit) {
    switch(unit) {
        case "byte":
            return value => value
        case "kilobyte":
            return value => value * 1024
        case "megabyte":
            return value => value * 1024 * 1024
        case "gigabyte":
            return value => value * 1024 * 1024 * 1024
    }
}
const memoryUnitSelect = document.querySelector("#histogram-props select#memory-unit")
if (memoryUnitSelect) {
    setInputValueFromSearchParam(memoryUnitSelect)
    TRANSFORMATION = memoryUnitToByte(memoryUnitSelect.value)
    memoryUnitSelect.addEventListener("change", event => {
        let memoryUnit = event.target.value
        TRANSFORMATION = memoryUnitToByte(memoryUnit)
    
        let input = event.target
        setURLSearchParam(input.name, input.value)
        buildChart(TRANSFORMATION)
    })
}

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!! Timing distribution specific !!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

function timeUnitToNanos(unit) {
    switch(unit) {
        case "nanoseconds":
            return value => value
        case "microseconds":
            return value => value * 1000
        case "milliseconds":
            return value => value * 1000 * 1000
    }
}

const baseMax = 1000 * 1000 * 1000 * 60 * 10
const timeUnitToMaxValue = {
    "nanoseconds": baseMax,
    "microseconds": timeUnitToNanos("microseconds")(baseMax),
    "milliseconds": timeUnitToNanos("milliseconds")(baseMax),
}
const timeUnitSelect = document.querySelector("#histogram-props select#time-unit")
const maxValueInput = document.getElementById("maximum-value")
if (timeUnitSelect) {
    setInputValueFromSearchParam(timeUnitSelect)
    TRANSFORMATION = timeUnitToNanos(timeUnitSelect.value)
    timeUnitSelect.addEventListener("change", event => {
        let timeUnit = event.target.value
        maxValueInput.value = timeUnitToMaxValue[timeUnit]
        TRANSFORMATION = timeUnitToNanos(timeUnit)
    
        let input = event.target
        setURLSearchParam(input.name, input.value)
        buildChart(TRANSFORMATION)
    })
}

// !!!!!!!!!!!!!!!!!!!!!!!!!
// !!! Custom data modal !!!
// !!!!!!!!!!!!!!!!!!!!!!!!!

// Open custom data modal when custom data option is selected
const customDataInput = document.getElementById("custom-data-input-group")
customDataInput && customDataInput.addEventListener('click', () => {
    customDataModalOverlay.style.display = "block"
    fillUpCustomDataWithDummyData()
})

// Rebuild chart everytime the custom data text is changed
const customDataTextarea = document.querySelector("#custom-data-modal textarea")
customDataTextarea && customDataTextarea.addEventListener("change", () => buildChart(TRANSFORMATION))

// Close modal when we click the overlay
const customDataModalOverlay = document.getElementById("custom-data-modal-overlay")
customDataModalOverlay && customDataModalOverlay.addEventListener('click', () => {
    customDataModalOverlay.style.display = "none"
})

// We need to stop propagation for click events on the actual modal,
// so that clicking it doesn't close it
const customDataModal = document.getElementById("custom-data-modal")
customDataModal && customDataModal.addEventListener("click", event => event.stopPropagation())

// !!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!! Data options events !!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!

const options = document.querySelectorAll("#data-options input")
options.forEach(option => {
    option.addEventListener("change", event => {
        event.preventDefault()

        let input = event.target
        setURLSearchParam(input.name, input.value)
        buildChart(TRANSFORMATION)
    })

    if (searchParams().get(option.name) == option.value) {
        option.checked = true

        // We won't save the custom data in the URL,
        // if that is the value on load, we create dummy data
        if (option.value == "custom") {
            fillUpCustomDataWithDummyData()
        }
    }
})

// !!!!!!!!!!!!!!!!!!!!!!!
// !!! Histogram props !!!
// !!!!!!!!!!!!!!!!!!!!!!!

const inputs = [
    ...document.querySelectorAll(`#histogram-props input`),
    document.querySelector(`#histogram-props select#kind`)
]

inputs.forEach(input => {
    setInputValueFromSearchParam(input)
    input.addEventListener("change", event => {
        let input = event.target
        setURLSearchParam(input.name, input.value)
        buildChart(TRANSFORMATION)
    })
})

// Build the chart once we are done loading field values
// If we are not in a memory distribution simulator,
// the tranformation function will do nothing
buildChart(TRANSFORMATION)

