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
customDataTextarea && customDataTextarea.addEventListener("change", () => buildChart())

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
        buildChart()
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
        buildChart()
    })
})

// Build the chart once we are done loading field values
buildChart()
