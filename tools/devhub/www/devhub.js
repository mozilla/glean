// Code powering "developer dashboard" aka devhub, at <https://mozilla.github.io/glean/devhub>.
// Based on <https://github.com/tigerbeetle/tigerbeetle/blob/b6d541562290f23c10760ea20b559cf21b9010b0/src/devhub/devhub.js>
//
// SPDX-License-Identifier: Apache-2.0

window.onload = () => {
  main_metrics();
  setup_filter();
}

function debounce(ms, fn) {
  let timer
  return (...args) => {
    clearTimeout(timer)
    timer = setTimeout(() => fn(...args), ms)
  }
}

function setup_filter() {
  const filterbox = document.getElementById("filter");

  filterbox.onkeyup = debounce(100, () => {
    const q = filterbox.value;
    const re = new RegExp(q, "i");
    document.querySelectorAll("#charts > div").forEach(e => {
      if (!q) {
        e.style.display = "block";
        return;
      }

      const title = e.querySelector("text")?.innerHTML;
      if (!q || title?.search(re) >= 0) {
        e.style.display = "block";
      } else {
        e.style.display = "none";
      }
    });
  });
}

async function main_metrics() {
  const data_url = "https://raw.githubusercontent.com/mozilla/glean-devhubdb/main/devhub/data.json";;
  const data = await (await fetch(data_url)).text();
  const max_batches = 200;
  const batches = data.split("\n")
    .filter((it) => it.length > 0)
    .map((it) => JSON.parse(it))
    .slice(-1 * max_batches)
    .reverse();

  const series = batches_to_series(batches);
  plot_series(series, document.querySelector("#charts"), batches.length);
}

function format_duration(duration_ms) {
  const milliseconds = duration_ms % 1000;
  const seconds = Math.floor((duration_ms / 1000) % 60);
  const minutes = Math.floor((duration_ms / (1000 * 60)) % 60);
  const hours = Math.floor((duration_ms / (1000 * 60 * 60)) % 24);
  const days = Math.floor(duration_ms / (1000 * 60 * 60 * 24));
  const parts = [];

  if (days > 0) {
    parts.push(`${days}d`);
  }
  if (hours > 0) {
    parts.push(`${hours}h`);
  }
  if (minutes > 0) {
    parts.push(`${minutes}m`);
  }
  if (days == 0) {
    if (seconds > 0 || parts.length === 0) {
      parts.push(`${seconds}s`);
    }
    if (hours == 0 && minutes == 0) {
      if (milliseconds > 0) {
        parts.push(`${milliseconds}ms`);
      }
    }
  }

  return parts.join(" ");
}

// The input data is array of runs, where a single run contains many measurements (eg, file size,
// build time).
//
// This function "transposes" the data, such that measurements with identical labels are merged to
// form a single array which is what we want to plot.
//
// This doesn't depend on particular plotting library though.
function batches_to_series(batches) {
  const results = new Map();
  for (const [index, batch] of batches.entries()) {
    for (const metric of batch.metrics) {
      const series = results.getOrInsertComputed(metric.name, (key) => {
        return {
          name: key,
          unit: undefined,
          value: [],
          git_commit: [],
          timestamp: [],
        };
      });

      if (!series.unit) {
        series.unit = metric.unit;
      }

      // Even though our x-axis is time (timestamp of the commit this ran on),
      // we want to spread things out evenly by batch,
      // rather than group according to time.
      // Apex charts is much quicker when given an x value, even though it isn't strictly needed.
      series.value.push([batches.length - index, metric.value]);
      series.git_commit.push(batch.attributes.git_commit);
      series.timestamp.push(batch.timestamp);
    }
  }

  return results;
}

// Plot time series using <https://apexcharts.com>.
function plot_series(series_list, root_node, batch_count) {
  for (const series of series_list.values()) {
    let options = {
      title: {
        text: series.name,
      },
      chart: {
        id: series.name,
        group: "devhub",
        type: "line",
        height: "400px",
        animations: {
          enabled: false,
        },
        events: {
          dataPointSelection: (event, chartContext, { dataPointIndex }) => {
            window.open(
              "https://github.com/mozilla/glean/commit/" +
                series.git_commit[dataPointIndex],
            );
          },
        },
      },
      markers: {
        size: 4,
      },
      series: [{
        name: series.name,
        data: series.value,
      }],
      xaxis: {
        categories: Array(series.value[series.value.length - 1][0]).fill("")
          .concat(
            series.timestamp.map((timestamp) =>
              format_date_day(new Date(timestamp * 1000))
            ).reverse(),
          ),
        min: 0,
        max: batch_count,
        tickAmount: 15,
        axisTicks: {
          show: false,
        },
        tooltip: {
          enabled: false,
        },
      },
      tooltip: {
        enabled: true,
        shared: false,
        intersect: true,
        x: {
          formatter: function (val, { dataPointIndex }) {
            const formattedDate = format_date_day_time(
              new Date(series.timestamp[dataPointIndex] * 1000),
            );
            return `<div>${
              series.git_commit[dataPointIndex]
            }</div><div>${formattedDate}</div>`;
          },
        },
      },
    };

    if (series.unit === "bytes") {
      options.yaxis = {
        labels: {
          formatter: format_bytes,
        },
      };
    }

    if (series.unit === "ms") {
      options.yaxis = {
        labels: {
          formatter: format_duration,
        },
      };
    }

    const div = document.createElement("div");
    root_node.append(div);
    const chart = new ApexCharts(div, options);
    chart.render();
  }
}

function format_bytes(bytes) {
  if (bytes === 0) return "0 Bytes";

  const k = 1024;
  const sizes = [
    "Bytes",
    "KiB",
    "MiB",
    "GiB",
    "TiB",
    "PiB",
    "EiB",
    "ZiB",
    "YiB",
  ];

  let i = 0;
  while (i != sizes.length - 1 && Math.pow(k, i + 1) < bytes) {
    i += 1;
  }

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
}

function format_date_day(date) {
  return format_date(date, false);
}

function format_date_day_time(date) {
  return format_date(date, true);
}

function format_date(date, include_time) {
  const pad = (number) => String(number).padStart(2, "0");

  const year = date.getFullYear();
  const month = pad(date.getMonth() + 1); // Months are 0-based.
  const day = pad(date.getDate());
  const hours = pad(date.getHours());
  const minutes = pad(date.getMinutes());
  const seconds = pad(date.getSeconds());
  return include_time
    ? `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`
    : `${year}-${month}-${day}`;
}
