// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

"use strict"

// Do not show dataset legend for graph,
// defining this in the Chart options doesn't seem to work
Chart.defaults.global.legend = false;

const DATA_SAMPLE_COUNT = 20000;

/**
 * Build and replace the previous chart with a new one.
 *
 * @param {String} kind The kind of histogram that should be build, possible values are "functional", "exponential" or "linear"
 * @param {Object} props The properties related to the given histogram, keys differ based in the kind
 * @param {String} dataOption The chosen way to build data, possible values are "normally-distributed", "log-normally-distributed", "uniformly-distributed" or "custom"
 * @param {String} customData In case `dataOption` is "custom", this should contain a String containing a JSON array of numbers
 * @param {HTMLElement} legend The HTML element that should contain the text of the chart legend
 * @param {HTMLElement} chartSpace The HTML element that should contain the chart
 * @param {function} transformation Option function to be applied to generated values
 */
function buildChart (kind, props, dataOption, customData, chartLegend, chartSpace, transformation) {
    const { buckets, data, percentages, mean } = buildData(kind, props, dataOption, customData, transformation);

    if (kind != "functional") {
        chartLegend.innerHTML = `Using these parameters, the widest bucket's width is <b>${getWidestBucketWidth(buckets)}</b>.`;
    } else {
        chartLegend.innerHTML = `
            Using these parameters, the maximum bucket is <b>${buckets[buckets.length - 1]}</b>.
            <br /><br />
            The mean of the recorded data is <b>${formatNumber(mean)}</b>.
        `;
    }

    // Clear chart for re-drawing,
    // here we need to re-create the whole canvas
    // otherwise we keep rebuilding the new graph on top of the previous
    // and that causes hover madness
    const canvas = document.createElement("canvas");
    chartSpace.innerHTML = "";
    chartSpace.appendChild(canvas);
    // Draw the chart
    const ctx = canvas.getContext("2d");
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
                        beginAtZero: true,
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
                        beginAtZero: true,
                        callback: (value, index, values) => {
                            const interval = Math.floor(values.length / 25)
                            if (interval > 0 && index % interval != 0) {
                                return ""
                            } else {
                                return value
                            }
                        }
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
                        if (kind == "functional") {
                            return index == lastIndex ? `${value} samples (${percentage}%) where sample value > ${buckets[lastIndex]} (overflow)`
                                : `${value} samples (${percentage}%) where ${buckets[index]} ≤ sample value < ${buckets[index + 1]}`
                        } else {
                            return index == 0 ? `${value} samples (${percentage}%) where sample value < ${buckets[0]} (underflow)`
                                : index == lastIndex ? `${value} samples (${percentage}%) where sample value > ${buckets[lastIndex]} (overflow)`
                                : `${value} samples (${percentage}%) where ${buckets[index]} ≤ sample value < ${buckets[index + 1]}`
                        }
                    },
                }
            }
        }
    });
}

/**
 * Build the data to be rendered in the charts.
 *
 * @param {String} kind The kind of histogram that should be build, possible values are "functional", "exponential" or "linear"
 * @param {Object} props The properties related to the given histogram, keys differ based in the kind
 * @param {String} dataOption The chosen way to build data, possible values are "normally-distributed", "log-normally-distributed", "uniformly-distributed" or "custom"
 * @param {String} customData In case `dataOption` is "custom", this should contain a String containing a JSON array of numbers
 * @param {function} transformation Option function to be applied to generated values
 *
 * @returns {Object} An object containing the bucket and values of a histogram
 */
function buildData (kind, props, dataOption, customData, transformation) {
    if (kind == "functional") {
        return buildDataFunctional(props, dataOption, customData, transformation);
    } else {
        return buildDataPreComputed(kind, props, dataOption, customData, transformation);
    }
}

/**
 * Build sample data or parse custom data.
 *
 * @param {String} dataOption The chosen way to build data, possible values are "normally-distributed", "log-normally-distributed", "uniformly-distributed" or "custom"
 * @param {String} customData In case `dataOption` is "custom", this should contain a String containing a JSON array of numbers
 * @param {Number} lower The lowest number the generated values may be, defaults to `1`
 * @param {Number} upper The highest number the generated values may be, defaults to `100`
 *
 * @returns {Array} An array of values, this array has DATA_SAMPLE_COUNT length if not custom
 */
function buildSampleData (dataOption, customData, lower, upper) {
    if (!lower) lower = 1;
    if (!upper) upper = 100;
    const values =
        dataOption == "normally-distributed" ? normalRandomValues((lower + upper) / 2, (upper - lower) / 8, DATA_SAMPLE_COUNT)
        : dataOption == "log-normally-distributed" ? logNormalRandomValues(Math.sqrt(Math.max(lower, 1) * upper), Math.pow(upper / Math.max(lower, 1), 1 / 8), DATA_SAMPLE_COUNT)
        : dataOption == "uniformly-distributed" ? uniformValues(lower, upper, DATA_SAMPLE_COUNT)
        : parseJSONString(customData);
    return values;
}

/**
 * Build the data to be rendered in the charts, in case histogram kind is "exponential" or "linear".
 *
 * @param {String} kind The kind of histogram that should be build, possible values are "functional", "exponential" or "linear"
 * @param {Object} props The properties related to the given histogram, keys differ based in the kind
 * @param {String} dataOption The chosen way to build data, possible values are "normally-distributed", "log-normally-distributed", "uniformly-distributed" or "custom"
 * @param {String} customData In case `dataOption` is "custom", this should contain a String containing a JSON array of numbers
 * @param {function} transformation Optional function to be applied to generated values
 *
 * @returns {Object} An object containing the bucket and values of a histogram
 */
function buildDataPreComputed (kind, props, dataOption, customData, transformation = v => v) {
    const { lowerBound, upperBound, bucketCount } = props;
    const buckets = kind == "exponential"
        ? exponentialRange(lowerBound, upperBound, bucketCount)
        : linearRange(lowerBound, upperBound, bucketCount);

    const lowerBucket = buckets[0];
    const upperBucket = buckets[buckets.length - 1];
    const values = buildSampleData(dataOption, customData, lowerBucket, upperBucket)
        .map(v => transformation(v));

    const data = accumulateValuesIntoBucketsPreComputed(buckets, values)
    return {
        data,
        buckets,
        percentages: data.map(v => v * 100 / values.length),
    };
}

/**
 * Build the data to be rendered in the charts, in case histogram kind is "functional".
 *
 * @param {String} kind The kind of histogram that should be build, possible values are "functional", "exponential" or "linear"
 * @param {Object} props The properties related to the given histogram, keys differ based in the kind
 * @param {String} dataOption The chosen way to build data, possible values are "normally-distributed", "log-normally-distributed", "uniformly-distributed" or "custom"
 * @param {String} customData In case `dataOption` is "custom", this should contain a String containing a JSON array of numbers
 * @param {function} transformation Optional function to be applied to generated values
 *
 * @returns {Object} An object containing the bucket and values of a histogram
 */
function buildDataFunctional(props, dataOption, customData, transformation = v => v) {
    const { logBase, bucketsPerMagnitude, maximumValue } = props;
    const values = buildSampleData(dataOption, customData)
        .map(v => transformation(v));
    const acc = accumulateValuesIntoBucketsFunctional(logBase, bucketsPerMagnitude, maximumValue, values);
    const data = Object.values(acc)
    return {
        data,
        buckets: Object.keys(acc),
        percentages: data.map(v => v * 100 / values.length),
        mean: values.reduce((sum, current) => sum + current) / values.length
    };
}

/**
 * Get the search params of the current URL.
 *
 * @returns {URLSearchParams} The search params object related to the current URL
 */
function searchParams() {
    return (new URL(document.location)).searchParams;
}

/**
 * Add a new param to the current pages URL, no relaoding
 *
 * @param {String} name The name of the param to set
 * @param {String} value The value of the param to set
 */
function setURLSearchParam(name, value) {
    let params = searchParams();
    params.set(name, value);
    history.pushState(null, null, `?${params.toString()}`);
}

/**
 * Attempts to get a search param in the current pages URL with the same name as a given input,
 * if such a param exists, set the value of the given input to the same value as the param found.
 *
 * @param {HTMLElement} input The input to update
 */
function setInputValueFromSearchParam(input) {
    let param = searchParams().get(input.name);
    if (param) input.value = param;
}

/**
 * Finds the widest bucket in a list of buckets.
 *
 * The width of a bucket is defined by it's minimum value minus the previous buckets minimum value.
 *
 * @param {Array} buckets An array of buckets
 *
 * @returns {Number} The length of the widest bucket found
 */
function getWidestBucketWidth (buckets) {
    let widest = 0;
    for (let i = 1; i < buckets.length; i++) {
        const currentWidth = buckets[i] - buckets[i - 1];
        if (currentWidth > widest) {
            widest = currentWidth;
        }
    }
    return widest;
}

/**
 * Attemps to parse a string as JSON, if unsuccesfull returns an empty array.
 *
 * @param {String} data A string containing a JSON encoded array
 *
 * @returns {Array} The parsed array
 */
function parseJSONString (data) {
    let result = [];
    try {
        result = JSON.parse(data);
    } finally {
        return result;
    }
}

/**
 * Fills up a given textarea with dummy data.
 *
 * @param {HTMLElement} textarea The textarea to fill up
 */
function fillUpTextareaWithDummyData (textarea) {
    const lower = 1;
    const upper = 100;
    const dummyData = logNormalRandomValues(Math.sqrt(Math.max(lower, 1) * upper), Math.pow(upper / Math.max(lower, 1), 1 / 8), DATA_SAMPLE_COUNT);
    const prettyDummyData = JSON.stringify(dummyData, undefined, 4);
    textarea.value = prettyDummyData;
}

/**
 * Precomputes the buckets for an exponential histogram.
 *
 * This is copied and adapted from glean-core/src/histograms/exponential.rs
 *
 * @param {Number} min The minimum value that can be recorded on this histogram
 * @param {Number} max The maximum value that can be recorded on this histogram
 * @param {Number} bucketCount The number of buckets on this histogram
 *
 * @return {Array} The array of calculated buckets
 */
function exponentialRange (min, max, bucketCount) {
    let logMax = Math.log(max);

    let ranges = [0];
    let current = min;
    if (current == 0) {
        current = 1;
    }
    ranges.push(current);

    for (let i = 2; i < bucketCount; i++) {
        let logCurrent = Math.log(current);
        let logRatio = (logMax - logCurrent) / (bucketCount - i);
        let logNext = logCurrent + logRatio;
        let nextValue = Math.round(Math.exp(logNext));
        current = nextValue > current ? nextValue : current + 1;
        ranges.push(current);
    }

    return ranges;
}

/**
 * Precomputes the buckets for an exponential histogram.
 *
 * This is copied and adapted from glean-core/src/histograms/linear.rs
 *
 * @param {Number} min The minimum value that can be recorded on this histogram
 * @param {Number} max The maximum value that can be recorded on this histogram
 * @param {Number} bucketCount The number of buckets on this histogram
 *
 * @return {Array} The array of calculated buckets
 */
function linearRange (min, max, bucketCount) {
    let ranges = [0];
    min = Math.max(1, min);
    for (let i = 1; i < bucketCount; i++) {
        let range = Math.round((min * (bucketCount - 1 - i) + max * (i - 1)) / (bucketCount - 2));
        ranges.push(range);
    }

    return ranges;
}


/**
 * Accumulate an array of values into buckets for histograms with pre-computed buckets
 *
 * @param {Array} buckets An array of buckets for a given histogram
 * @param {Array} values The values to be recorded on this histogram
 *
 * @return {Array} The array of recorded values
 */
function accumulateValuesIntoBucketsPreComputed (buckets, values) {
    let result = new Array(buckets.length).fill(0);
    for (const value of values) {
        let placed = false;
        for (let i = 0; i < buckets.length - 1; i++) {
            if (buckets[i] <= value && value < buckets[i + 1]) {
                placed = true;
                result[i]++;
                break;
            }
        }
        // If the value was not placed it is after the buckets limit,
        // thus it fits in the last bucket
        if (!placed) {
            result[result.length - 1]++;
        }
    }

    return result;
}

/**
 * Accumulate an array of values into buckets for histograms with dinamically created buckets.
 *
 * For these types of histograms bucketing is performed by a function, rather than pre-computed buckets.
 * The bucket index of a given sample is determined with the following function:
 *
 * i = ⌊n log<sub>base</sub>(𝑥)⌋
 *
 * In other words, there are n buckets for each power of `base` magnitude.
 *
 * Based on glean-core/src/histograms/functional.rs
 *
 * @param {Number} logBase The log base for the bucketing algorithm
 * @param {Array} bucketsPerMagnitude How many buckets to create per magnitude
 * @param {Number} maximumValue The maximum value that can be recorded on this histogram
 * @param {Array} values The values to be recorded on this histogram
 *
 * @return {Object} An object mapping buckets and recorded values of this histogram
 */
function accumulateValuesIntoBucketsFunctional (logBase, bucketsPerMagnitude, maximumValue, values) {
    const exponent = Math.pow(logBase, 1 / bucketsPerMagnitude);

    const sampleToBucketIndex = sample => Math.floor(log(sample + 1, exponent));;
    const bucketIndexToBucketMinimum = index => Math.floor(Math.pow(exponent, index));
    const sampleToBucketMinimum = sample => {
        let bucketMinimum;
        if (sample == 0) {
            bucketMinimum = 0;
        } else {
            const bucketIndex = sampleToBucketIndex(sample);
            bucketMinimum = bucketIndexToBucketMinimum(bucketIndex);
        }
        return bucketMinimum;
    }

    let result = {};
    let min, max;
    for (let value of values) {
        // Cap on the maximum value
        if (value > maximumValue) {
            value = maximumValue;
        }

        const bucketMinimum = String(sampleToBucketMinimum(value));
        if (!(bucketMinimum in result)) {
            result[bucketMinimum] = 0;
        }
        result[bucketMinimum]++;

        // Keep track of the max and min values accumulated,
        // we will need them later
        if (!min || value < min) min = value;
        if (!max || value > max) max = value;
    }

    // Fill in missing buckets,
    // this is based on the snapshot() function
    const minBucket = sampleToBucketIndex(min);
    const maxBucket = sampleToBucketIndex(max) + 1;
    for (let idx = minBucket; idx <= maxBucket; idx++) {
        let bucketMinimum = String(bucketIndexToBucketMinimum(idx));
        if (!(bucketMinimum in result)) {
            result[bucketMinimum] = 0;
        }
    }

    return result;
}

/**
 * Box-Muller transform in polar form.
 *
 * Values below zero will be truncated to 0.
 *
 * Copied over and adapted
 * from https://github.com/mozilla/telemetry-dashboard/blob/bd7c213391d4118553b9ff1791ed0441bf912c60/histogram-simulator/simulator.js
 *
 * @param {Number} mu
 * @param {Number} sigma
 * @param {Number} count The length of the generated array
 *
 * @return {Array} An array of generated values
 */
function normalRandomValues (mu, sigma, count) {
    let values = [];
    let z0, z1, value;
    for (let i = 0; values.length < count; i++) {
        if (i % 2 === 0) {
            let x1, x2, w;
            do {
                x1 = 2 * Math.random() - 1;
                x2 = 2 * Math.random() - 1;
                w = x1 * x1 + x2 * x2;
            } while (w >= 1)
            w = Math.sqrt((-2 * Math.log(w)) / w);
            z0 = x1 * w;
            z1 = x2 * w;
            value = z0;
        } else {
            value = z1;
        }
        value = value * sigma + mu;

        values.push(value);
    }
    return values.map(value => value >= 0 ? Math.floor(value) : 0);
}

/**
 * Box-Muller transform in polar form for log-normal distributions
 *
 * Values below zero will be truncated to 0.
 *
 * Copied over and adapted
 * from https://github.com/mozilla/telemetry-dashboard/blob/bd7c213391d4118553b9ff1791ed0441bf912c60/histogram-simulator/simulator.js
 *
 * @param {Number} mu
 * @param {Number} sigma
 * @param {Number} count The length of the generated array
 *
 * @return {Array} An array of generated values
 */
function logNormalRandomValues (mu, sigma, count) {
    let values = [];
    let z0, z1, value;
    for (let i = 0; i < count; i++) {
        if (i % 2 === 0) {
            let x1, x2, w;
            do {
                x1 = 2 * Math.random() - 1;
                x2 = 2 * Math.random() - 1;
                w = x1 * x1 + x2 * x2;
            } while (w >= 1)
            w = Math.sqrt((-2 * Math.log(w)) / w);
            z0 = x1 * w;
            z1 = x2 * w;
            value = z0;
        } else {
            value = z1;
        }
        value = Math.exp(value * Math.log(sigma) + Math.log(mu));

        values.push(value);
    }
    return values.map(value => value >= 0 ? Math.floor(value) : 0);
}

/**
 * A uniformly distributed array of random values
 *
 * @param {Number} min The minimum value this function may generate
 * @param {Number} max The maximum value this function may generate
 * @param {Number} count The length of the generated array
 *
 * @return {Array} An array of generated values
 */
function uniformValues (min, max, count) {
    let values = [];
    for (var i = 0; i <= count; i++) {
        values.push(Math.random() * (max - min) + min);
    }

    return values;
}


/**
 * Formats a number as a string.
 *
 * Copied over and adapted
 * from https://github.com/mozilla/telemetry-dashboard/blob/bd7c213391d4118553b9ff1791ed0441bf912c60/histogram-simulator/simulator.js
 *
 * @param {Number} number The number to format
 *
 * @return {String} The formatted number
 */
function formatNumber(number) {
    if (number == Infinity) return "Infinity";
    if (number == -Infinity) return "-Infinity";
    if (isNaN(number)) return "NaN";

    const mag = Math.abs(number);
    const exponent =
        Math.log10 !== undefined ? Math.floor(Math.log10(mag))
            : Math.floor(Math.log(mag) / Math.log(10));
    const interval = Math.pow(10, Math.floor(exponent / 3) * 3);
    const units = {
        1000: "k",
        1000000: "M",
        1000000000: "B",
        1000000000000: "T"
    };

    if (interval in units) {
        return Math.round(number * 100 / interval) / 100 + units[interval];
    }

    return Math.round(number * 100) / 100;
}


/**
 * Arbitrary base log function, Javascript doesn't have one
 *
 * @param {Number} number A numeric expression
 * @param {base} base The log base
 *
 * @return {Number} The calculation result
 */
function log(number, base) {
    return Math.log(number) / Math.log(base);
}

