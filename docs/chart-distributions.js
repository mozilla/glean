// Do not show dataset legend for graph,
// defining this in the Chart options doesn"t seem to work
Chart.defaults.global.legend = false

const DATA_SAMPLE_COUNT = 20000

// !!!!!!!!!!!!!!!!!!!!!!
// !!! Chart building !!!
// !!!!!!!!!!!!!!!!!!!!!!

function buildChart () {
    const { buckets, data } = buildDataFromInputs()
    percentages = data.map(v => v * 100 / DATA_SAMPLE_COUNT)

    if (getCurrentHistogramKind() != "functional") {
        // Update chart legend
        const legend = document.getElementById("histogram-chart-legend")
        legend.innerHTML = `Using these parameters, the widest bucket's width is <b>${getWidestBucketWidth(buckets)}</b>.`
    }

    // Clear chart for re-drawing,
    // here we need to re-create the whole canvas
    // otherwise we keep rebuilding the new graph on top of the previous
    // and that causes hover madness
    const chartSpace = document.getElementById("histogram-chart")
    const canvas = document.createElement("canvas")
    chartSpace.innerHTML = ""
    chartSpace.appendChild(canvas)
    // Draw the chart
    const ctx = canvas.getContext("2d")
    new Chart(ctx, {
        type: "bar",
        data: {
            labels: buckets,
            datasets: [{
                barPercentage: .95,
                categoryPercentage: 1,
                backgroundColor: "rgba(76, 138, 196, 1)",
                hoverBackgroundColor: "rgba(0, 89, 171, 1)",
                data: percentages
            }],
        },
        options: {
            responsive: true,
            scales: {
                yAxes: [{
                    ticks: {
                        callback: value => `${value}%`
                    },
                    scaleLabel: {
                        display: true,
                        labelString: "Percentages of samples"
                    }
                }],
                xAxes: [{
                    ticks: {
                        autoSkip: false,
                        minRotation: 50,
                        maxRotation: 50,
                    },
                    scaleLabel: {
                        display: true,
                        labelString: "Buckets"
                    }
                }]
            },
            tooltips: {
                mode: "index",
                callbacks: {
                    title: () => null,
                    label: item => {
                        const index = item.index
                        const lastIndex = percentages.length - 1
                        const percentage = percentages[index].toFixed(2)
                        const value = formatNumber(data[index])
                        return index == 0 ? `${value} samples (${percentage}%) where sample value < ${buckets[0]} (underflow)`
                            : index == lastIndex ? `${value} samples (${percentage}%) where sample value > ${buckets[lastIndex]} (overflow)`
                            : `${value} samples (${percentage}%) where ${buckets[index - 1]} ≤ sample value < ${buckets[index]}`
                    },
                }
            }
        }
    })
}

// !!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!! Chart data building !!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!

function buildDataFromInputs () {
    let histogramKind = getCurrentHistogramKind()
    if (histogramKind == "functional") {
        return buildDataFromInputsFunctional()
    } else {
        return buildDataFromInputsPreComputed(histogramKind)
    }
}

function buildSampleData (lower, upper) {
    if (!lower) lower = 1
    if (!upper) upper = 100
    const dataOption = document.querySelector("#data-options input:checked").value
    const values =
        dataOption == "normally-distributed" ? normalRandomValues((lower + upper) / 2, (upper - lower) / 8, DATA_SAMPLE_COUNT)
        : dataOption == "log-normally-distributed" ? logNormalRandomValues(Math.sqrt(Math.max(lower, 1) * upper), Math.pow(upper / Math.max(lower, 1), 1 / 8), DATA_SAMPLE_COUNT)
        : dataOption == "uniformly-distributed" ? uniformValues(lower, upper, DATA_SAMPLE_COUNT)
        : parseCustomData()
    return values
}

function buildDataFromInputsPreComputed (kind) {
    // We need to do explicit type coersion here,
    // otherwise Javascript will sometimes figure that 1 + 1 = "11"
    const lowerBound = Number(document.getElementById("lower-bound").value)
    const upperBound = Number(document.getElementById("upper-bound").value)
    const bucketCount = Number(document.getElementById("bucket-count").value)

    const buckets = kind == "exponential"
        ? exponentialRange(lowerBound, upperBound, bucketCount)
        : linearRange(lowerBound, upperBound, bucketCount)

    const lowerBucket = buckets[0]
    const upperBucket = buckets[buckets.length - 1]
    const values = buildSampleData(lowerBucket, upperBucket)

    return {
        buckets,
        data: accumulateValuesIntoBucketsPreComputed(buckets, values),
    }
}

function buildDataFromInputsFunctional() {
    const logBase = Number(document.getElementById("log-base").value)
    const bucketsPerMagnitude = Number(document.getElementById("buckets-per-magnitude").value)
    const maximumValue = Number(document.getElementById("maximum-value").value || Number.MAX_SAFE_INTEGER)

    const values = buildSampleData()

    const acc = accumulateValuesIntoBucketsFunctional(logBase, bucketsPerMagnitude, maximumValue, values)
    return {
        buckets: Object.keys(acc),
        data: Object.values(acc)
    }
}

// !!!!!!!!!!!!!!!!!
// !!! Utilities !!!
// !!!!!!!!!!!!!!!!!

function searchParams() {
    return (new URL(document.location)).searchParams
}

function getCurrentHistogramKind() {
    const kind = document.getElementById("kind")
    return kind.value
}

function setURLSearchParam(name, value) {
    let params = searchParams()
    params.set(name, value)
    history.pushState(null, null, `?${params.toString()}`)
}

function setInputValueFromSearchParam(input) {
    let param = searchParams().get(input.name)
    if (param) input.value = param
}

function getWidestBucketWidth (buckets) {
    let widest = 0;
    for (let i = 1; i < buckets.length; i++) {
        const currentWidth = buckets[i] - buckets[i - 1]
        if (currentWidth > widest) {
            widest = currentWidth
        }
    }
    return widest
}

function parseCustomData () {
    let result = []
    try {
        const customData = document.querySelector("#custom-data-modal textarea").value
        result = JSON.parse(customData)
    } finally {
        return result
    }
}

function fillUpCustomDataWithDummyData () {
    const lower = 1
    const upper = 100
    const dummyData = logNormalRandomValues(Math.sqrt(Math.max(lower, 1) * upper), Math.pow(upper / Math.max(lower, 1), 1 / 8), DATA_SAMPLE_COUNT)
    const prettyDummyData = JSON.stringify(dummyData, undefined, 4);
    const customDataTextarea = document.querySelector("#custom-data-modal textarea")
    customDataTextarea.value = prettyDummyData;
}

// This is copied and adapted from glean-core/src/histograms/exponential.rs
function exponentialRange (min, max, bucketCount) {
    let logMax = Math.log(max)

    let ranges = [0]
    let current = min
    if (current == 0) {
        current = 1
    }
    ranges.push(current);

    for (let i = 2; i < bucketCount; i++) {
        let logCurrent = Math.log(current)
        let logRatio = (logMax - logCurrent) / (bucketCount - i)
        let logNext = logCurrent + logRatio
        let nextValue = Math.round(Math.exp(logNext))
        current = nextValue > current ? nextValue : current + 1
        ranges.push(current)
    }

    return ranges
}

// This is copied and adapted from glean-core/src/histograms/linear.rs
function linearRange (min, max, count) {
    let ranges = [0]
    min = Math.max(1, min)
    for (let i = 1; i < count; i++) {
        let range = Math.round((min * (count - 1 - i) + max * (i - 1)) / (count - 2))
        ranges.push(range)
    }

    return ranges
}

// Accumulate an array of values into buckets for histograms with pre-computed buckets
function accumulateValuesIntoBucketsPreComputed (buckets, values) {
    let result = new Array(buckets.length).fill(0)
    for (const value of values) {
        let placed = false
        for (let i = 0; i < buckets.length - 1; i++) {
            if (buckets[i] <= value && value < buckets[i + 1]) {
                placed = true
                result[i]++
                break
            }
        }
        // If the value was not placed it is after the buckets limit,
        // thus it fits in the last bucket
        if (!placed) {
            result[result.length - 1]++
        }
    }

    return result
}

// Based on glean-core/src/histograms/functional.rs
// Accumulate an array of values into buckets for histograms with dinamically created buckets
function accumulateValuesIntoBucketsFunctional (logBase, bucketsPerMagnitude, maximumValue, values) {
    const exponent = Math.pow(logBase, 1 / bucketsPerMagnitude)

    const sampleToBucketIndex = sample => Math.floor(log(sample + 1, exponent))
    const bucketIndexToBucketMinimum = index => Math.floor(Math.pow(exponent, index))
    const sampleToBucketMinimum = sample => {
        let bucketMinimum
        if (sample == 0) {
            bucketMinimum = 0
        } else {
            const bucketIndex = sampleToBucketIndex(sample)
            bucketMinimum = bucketIndexToBucketMinimum(bucketIndex)
        }
        return bucketMinimum
    }

    let result = {}
    let min, max
    for (let value of values) {
        // Cap on the maximum value
        if (value > maximumValue) {
            value = maximumValue
        }

        const bucketMinimum = String(sampleToBucketMinimum(value))
        if (!(bucketMinimum in result)) {
            result[bucketMinimum] = 0
        }
        result[bucketMinimum]++

        // Keep track of the max and min values accumulated,
        // we will need them later
        if (!min || value < min) min = value
        if (!max || value > max) max = value
    }

    // Fill in missing buckets,
    // this is based on the snapshot() function
    const minBucket = sampleToBucketIndex(min);
    const maxBucket = sampleToBucketIndex(max) + 1;
    for (let idx = minBucket; idx <= maxBucket; idx++) {
        let bucketMinimum = String(bucketIndexToBucketMinimum(idx));
        if (!(bucketMinimum in result)) {
            result[bucketMinimum] = 0
        }
    }

    return result
}

// Copied over and adapted
// from https://github.com/mozilla/telemetry-dashboard/blob/gh-pages/histogram-simulator/simulator.js
// Box-Muller transform in polar form
function normalRandomValues (mu, sigma, count) {
    let values = []
    let z0, z1, value
    for (let i = 0; values.length < count; i++) {
        if (i % 2 === 0) {
            let x1, x2, w
            do {
                x1 = 2 * Math.random() - 1
                x2 = 2 * Math.random() - 1
                w = x1 * x1 + x2 * x2
            } while (w >= 1)
            w = Math.sqrt((-2 * Math.log(w)) / w)
            z0 = x1 * w
            z1 = x2 * w
            value = z0
        } else {
            value = z1
        }
        value = value * sigma + mu

        values.push(value)
    }
    return values.map(value => value >= 0 ? value : 0)
}

// Copied over and adapted
// from https://github.com/mozilla/telemetry-dashboard/blob/gh-pages/histogram-simulator/simulator.js
// Box-Muller transform in polar form for log-normal distributions
function logNormalRandomValues (mu, sigma, count) {
    let values = []
    let z0, z1, value
    for (let i = 0; i < count; i++) {
        if (i % 2 === 0) {
            let x1, x2, w
            do {
                x1 = 2 * Math.random() - 1
                x2 = 2 * Math.random() - 1
                w = x1 * x1 + x2 * x2
            } while (w >= 1)
            w = Math.sqrt((-2 * Math.log(w)) / w)
            z0 = x1 * w
            z1 = x2 * w
            value = z0
        } else {
            value = z1
        }
        value = Math.exp(value * Math.log(sigma) + Math.log(mu))

        values.push(value)
    }
    return values.map(value => value >= 0 ? value : 0)
}

function uniformValues (min, max, count) {
    let values = []
    for (var i = 0; i <= count; i++) {
        values.push(Math.random() * (max - min) + min);
    }

    return values
}

// Copied over and adapted
// from https://github.com/mozilla/telemetry-dashboard/blob/gh-pages/histogram-simulator/simulator.js
function formatNumber(number) {
    if (number == Infinity) return "Infinity"
    if (number == -Infinity) return "-Infinity"
    if (isNaN(number)) return "NaN"

    const mag = Math.abs(number)
    const exponent =
        Math.log10 !== undefined ? Math.floor(Math.log10(mag))
            : Math.floor(Math.log(mag) / Math.log(10))
    const interval = Math.pow(10, Math.floor(exponent / 3) * 3)
    const units = {
        1000: "k",
        1000000: "M",
        1000000000: "B",
        1000000000000: "T"
    }

    if (interval in units) {
        return Math.round(number * 100 / interval) / 100 + units[interval]
    }

    return Math.round(number * 100) / 100
}

// Arbitrary base log function, Javascript doesn't have one
function log(number, base) {
    return Math.log(number) / Math.log(base);
}

