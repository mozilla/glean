// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

"use strict"

if (document.getElementById("histogram-chart")) {
    // Transformation function we may want to apply to each value before plotting
    //
    // The current use cases are memory distributions and timing distribution,
    // which may receive the values in a given unit, but transform them to a base one upon recording
    let TRANSFORMATION;

    function memoryUnitToByte(unit) {
        switch(unit) {
            case "byte":
                return value => value;
            case "kilobyte":
                return value => value * Math.pow(2, 10);
            case "megabyte":
                return value => value * Math.pow(2, 20);
            case "gigabyte":
                return value => value * Math.pow(2, 30);
        }
    }

    const memoryUnitSelect = document.querySelector("#histogram-props select#memory-unit")
    if (memoryUnitSelect) {
        setInputValueFromSearchParam(memoryUnitSelect);
        TRANSFORMATION = memoryUnitToByte(memoryUnitSelect.value);
        memoryUnitSelect.addEventListener("change", event => {
            let memoryUnit = event.target.value;
            TRANSFORMATION = memoryUnitToByte(memoryUnit);

            let input = event.target;
            setURLSearchParam(input.name, input.value);
            buildChartFromInputs();
        })
    }

    function timeUnitToNanos(unit) {
        switch(unit) {
            case "nanoseconds":
                return value => value;
            case "microseconds":
                return value => value * 1000;
            case "milliseconds":
                return value => value * 1000 * 1000;
        }
    }

    const baseMax = 1000 * 1000 * 1000 * 60 * 10;
    const timeUnitToMaxValue = {
        "nanoseconds": baseMax,
        "microseconds": timeUnitToNanos("microseconds")(baseMax),
        "milliseconds": timeUnitToNanos("milliseconds")(baseMax),
    };
    const timeUnitSelect = document.querySelector("#histogram-props select#time-unit");
    const maxValueInput = document.getElementById("maximum-value");
    if (timeUnitSelect) {
        setInputValueFromSearchParam(timeUnitSelect);
        TRANSFORMATION = timeUnitToNanos(timeUnitSelect.value);
        timeUnitSelect.addEventListener("change", event => {
            let timeUnit = event.target.value;
            maxValueInput.value = timeUnitToMaxValue[timeUnit];
            TRANSFORMATION = timeUnitToNanos(timeUnit);
        
            let input = event.target;
            setURLSearchParam(input.name, input.value);
            buildChartFromInputs();
        })
    }

    // Open custom data modal when custom data option is selected
    const customDataInput = document.getElementById("custom-data-input-group");
    customDataInput.addEventListener('click', () => {
        customDataModalOverlay.style.display = "block";
        const customDataTextarea = document.querySelector("#custom-data-modal textarea");
        if (!customDataTextarea.value) fillUpTextareaWithDummyData(customDataTextarea);
    })

    // Rebuild chart everytime the custom data text is changed
    const customDataTextarea = document.querySelector("#custom-data-modal textarea");
    customDataTextarea.addEventListener("change", () => buildChartFromInputs());

    // Close modal when we click the overlay
    const customDataModalOverlay = document.getElementById("custom-data-modal-overlay");
    customDataModalOverlay && customDataModalOverlay.addEventListener('click', () => {
        customDataModalOverlay.style.display = "none";
    });

    // We need to stop propagation for click events on the actual modal,
    // so that clicking it doesn't close it
    const customDataModal = document.getElementById("custom-data-modal");
    customDataModal.addEventListener("click", event => event.stopPropagation());

    const options = document.querySelectorAll("#data-options input");
    options.forEach(option => {
        option.addEventListener("change", event => {
            event.preventDefault();

            let input = event.target;
            setURLSearchParam(input.name, input.value);
            buildChartFromInputs();
        });

        if (searchParams().get(option.name) == option.value) {
            option.checked = true;

            // We won't save the custom data in the URL,
            // if that is the value on load, we create dummy data
            if (option.value == "custom") {
                const customDataTextarea = document.querySelector("#custom-data-modal textarea");
                fillUpTextareaWithDummyData(customDataTextarea);
            }
        }
    });

    const inputs = [
        ...document.querySelectorAll("#histogram-props input"),
        document.querySelector("#histogram-props select#kind")
    ];

    inputs.forEach(input => {
        setInputValueFromSearchParam(input);
        input.addEventListener("change", event => {
            let input = event.target;
            setURLSearchParam(input.name, input.value);
            buildChartFromInputs();
        });
    });

    buildChartFromInputs();

    /**
     * Build and replace the previous chart with a new one, based on the page inputs.
     */
    function buildChartFromInputs() {
        const kind = document.getElementById("kind").value

        let props;
        if (kind == "functional") {
            const logBase = Number(document.getElementById("log-base").value);
            const bucketsPerMagnitude = Number(document.getElementById("buckets-per-magnitude").value);
            const maximumValue = Number(document.getElementById("maximum-value").value || Number.MAX_SAFE_INTEGER);
            props = {
                logBase,
                bucketsPerMagnitude,
                maximumValue
            }
        } else {
            const lowerBound = Number(document.getElementById("lower-bound").value);
            const upperBound = Number(document.getElementById("upper-bound").value);
            const bucketCount = Number(document.getElementById("bucket-count").value);
            props = {
                lowerBound,
                upperBound,
                bucketCount
            }
        }

        buildChart(
            kind,
            props,
            document.querySelector("#data-options input:checked").value,
            document.querySelector("#custom-data-modal textarea").value,
            document.getElementById("histogram-chart-legend"),
            document.getElementById("histogram-chart"),
            TRANSFORMATION
        )
    }
}

