#!/usr/bin/env python3

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import argparse
import csv
import datetime
import os
import sys
import xml.etree.ElementTree as ET


from matplotlib import pyplot as plt


from config import *


"""
Get the data from this query:
https://sql.telemetry.mozilla.org/queries/66682
"""

# TODO: This script is likely to be obsoleted by bug 1602824


ROW_HEIGHT = 10


ZERO_LENGTH = 1
DUPLICATE_TIME = 2
NO_BASELINE = 3
MISSING_SEQ = 4
DUPLICATE_SEQ = 5
TOO_LATE = 6
FAR_FROM_4AM = 7
MAX_NOTES = 8

NOTE_SUMMARIES = {
    ZERO_LENGTH: "metrics ping had start/end time of < 1 minute",
    DUPLICATE_TIME: "2 or more metrics pings were collected within the same minute",
    NO_BASELINE: "a metrics ping was collected with no baseline ping since the last metrics ping",
    MISSING_SEQ: "the seq number is not contiguous with the previous ping",
    DUPLICATE_SEQ: "the same seq number was used more than once",
    TOO_LATE: "the metrics ping was collected more than 24 hours after the last baseline ping",
    FAR_FROM_4AM: "the metrics ping was sent more than an hour from 4am local time",
}


def load_data(filename):
    """
    Load the csv file and convert it to a list of dicts.
    """
    print("Loading CSV")
    data = []
    with open(filename) as fd:
        reader = csv.reader(fd)
        column_names = next(reader)
        for row in reader:
            data.append(dict((name, value) for (name, value) in zip(column_names, row)))
    return data


def parse_version(build_id):
    """
    Parse the "date-like" string out of the Fenix Nightly version convention.

    Returns `None` if no version found.
    """
    if build_id.startswith('"'):
        build_id = build_id[1:-1]
    if build_id.startswith("Nightly"):
        parts = build_id.split()
        date = int(parts[1])
        return date
    return None


def filter_data(data):
    """
    Remove pings that are too old.
    """
    now = datetime.datetime.now() + datetime.timedelta(days=1)
    result = [
        x
        for x in data
        if (
            get_local_time(x["start_time"]) >= FIRST_DATE
            and get_local_time(x["end_time"]) >= FIRST_DATE
            and get_local_time(x["start_time"]) < now
            and get_local_time(x["end_time"]) < now
        )
    ]

    print(f"Removed {len(data)-len(result)}/{len(data)} pings with out-of-range dates")
    return result


def annotate_data(data):
    """
    Add some derived values to the data set.
    """
    print("Annotating CSV")
    for ping in data:
        ping["start_time_tz"] = get_timezone(ping["start_time"])
        ping["end_time_tz"] = get_timezone(ping["end_time"])
        ping["start_time_local"] = get_local_time(ping["start_time"])
        ping["end_time_local"] = get_local_time(ping["end_time"])
        ping["start_time_hour"] = get_fractional_hour(ping["start_time_local"])
        ping["end_time_hour"] = get_fractional_hour(ping["end_time_local"])
        ping["version_date"] = parse_version(ping["app_version"])
        ping["notes"] = set()


def sort_data_by_client_id(data):
    """
    Reorganize the data so it is grouped by client id.
    """
    data_by_client_id = {}
    for row in data:
        client_id = row.get("client_id")
        data_by_client_id.setdefault(client_id, [])
        data_by_client_id[client_id].append(row)
    return data_by_client_id


def get_timezone(date_string):
    """
    Get the timezone offset from a Glean timestamp.
    """
    return date_string[-6:]


def get_local_time(date_string):
    """
    Get just the local time from the Glean timestamp.
    """
    return datetime.datetime.fromisoformat(date_string[:-6])


def get_fractional_hour(dt):
    """
    Convert the timestamp to a "fractional hour" (hours since the UNIX epoch)
    which is useful for plotting.
    """
    return dt.timestamp() / 360.0


def has_timezone_change(client_data):
    """
    Determine if the client had a timezone change in their history. These are
    excluded from the analysis for now because it's a complicated corner case.
    """
    timezones = set()
    for entry in client_data:
        timezones.add(entry["start_time_tz"])
        timezones.add(entry["end_time_tz"])
    return len(timezones) > 1


def organize_plot(data):
    """
    Organize the data into rows so no two timespans overlap.
    """
    rows = []

    for entry in data:
        # Find the first row will the entry will fit, otherwise, create a new
        # row
        for row in rows:
            if entry["start_time_local"] > row[-1]["end_time_local"]:
                row.append(entry)
                break
        else:
            rows.append([entry])

    return rows


def draw_line(parent, x1, x2, y1, y2, **kwargs):
    """
    Draw an SVG line. It is adjusted so it's length is at least 0.5 pixels,
    otherwise it will disappear during rendering.
    """
    diff = abs(x2 - x1) - 0.5
    if diff < 0:
        x1 -= diff / 2.0
        x2 += diff / 2.0
    attrs = {"x1": str(x1), "x2": str(x2), "y1": str(y1), "y2": str(y2)}
    kwargs = dict((k.replace("_", "-"), v) for (k, v) in kwargs.items())
    attrs.update(kwargs)
    return ET.SubElement(parent, "line", attrs)


def draw_text(parent, x, y, text, **kwargs):
    """
    Draw SVG text.
    """
    title = kwargs.pop("title", None)

    attrs = {"x": str(x), "y": str(y), "font-family": "sans-serif", "font-size": "10px"}
    kwargs = dict((k.replace("_", "-"), v) for (k, v) in kwargs.items())
    attrs.update(kwargs)
    el = ET.SubElement(parent, "text", attrs)
    el.text = text

    if title is not None:
        title_el = ET.SubElement(el, "title")
        title_el.text = title

    return el


def plot_timeline(client_id, data, metrics_rows, baseline_rows):
    """
    Make the SVG timeline.
    """
    data = sorted(data, key=lambda x: x["start_time_hour"])

    # Find the date range to determine the size of the plot
    min_time = data[0]["start_time_hour"]
    max_time = max(ping["end_time_hour"] for ping in data)
    width = max_time - min_time
    height = (len(metrics_rows) + len(baseline_rows) + 2) * ROW_HEIGHT

    svg = ET.Element(
        "svg",
        {
            "version": "1.1",
            "width": str(width),
            "height": str(height),
            "xmlns": "http://www.w3.org/2000/svg",
        },
    )
    ET.SubElement(
        svg,
        "rect",
        {
            "x": "0",
            "y": "0",
            "width": str(width),
            "height": str(height),
            "fill": "white",
        },
    )

    # Draw vertical lines at midnight and 4am, with the date indicated
    dt = data[0]["start_time_local"].replace(hour=0, minute=0, second=0)
    while get_fractional_hour(dt) < max_time:
        x = get_fractional_hour(dt) - min_time
        draw_line(svg, x, x, 0, height, stroke="#cccccc")
        draw_text(svg, x + 2, height - 2, dt.strftime("%m-%d"))

        four = dt.replace(hour=4)
        x = get_fractional_hour(four) - min_time
        draw_line(svg, x, x, 0, height, stroke="#cccccc", stroke_dasharray="2,1")

        dt += datetime.timedelta(days=1)

    # Draw markers for the first time key "FIX" versions were seen in the ping metadata
    fixes = list(enumerate(FIXES))
    for ping in sorted(data, key=lambda x: x["end_time_local"]):
        if ping["version_date"] is not None and ping["version_date"] >= fixes[0][1][1]:
            x = ping["end_time_hour"] - min_time
            draw_line(svg, x, x, 0, height, stroke="#33aa33")
            draw_text(svg, x + 2, 12, str(fixes[0][0] + 1), title=fixes[0][1][0])
            fixes.pop(0)

            if len(fixes) == 0:
                break

    # Draw the actual pings in the timeline
    y = ROW_HEIGHT
    for (rows, color) in ((baseline_rows, "#000088"), (metrics_rows, "#880000")):
        for row in rows[::-1]:
            for ping in row:
                draw_line(
                    svg,
                    ping["start_time_hour"] - min_time,
                    ping["end_time_hour"] - min_time,
                    y,
                    y,
                    stroke=color,
                    stroke_width="0.5",
                )

                if ping["ping_type"] == "baseline" and ping["duration"]:
                    session_start = (
                        get_fractional_hour(
                            ping["end_time_local"]
                            - datetime.timedelta(seconds=int(ping["duration"]))
                        )
                        - min_time
                    )
                    draw_line(
                        svg,
                        session_start,
                        ping["end_time_hour"] - min_time,
                        y,
                        y,
                        stroke=color,
                        stroke_width="3",
                    )

                if ping["notes"]:
                    x = 0
                    for note in sorted(list(ping["notes"])):
                        draw_text(
                            svg,
                            ping["end_time_hour"] - min_time + 2 + x,
                            y + 3,
                            str(note),
                            font_size="6px",
                            title=NOTE_SUMMARIES[note],
                        )
                        x += 8

            y += ROW_HEIGHT

    draw_text(svg, 2, 12, f"Android SDK: {data[0]['sdk']}")

    tree = ET.ElementTree(svg)

    with open(f"{client_id}.svg", "wb") as fd:
        tree.write(fd)


def find_issues(client_data, stats):
    """
    Find and notate issues for a client's data.
    """
    client_data = sorted(client_data, key=lambda x: (x["end_time_local"], x["seq"]))
    last_ping = None
    last_by_type = {}
    client_stats = {}
    for ping in client_data:
        # Find zero-length pings
        if (
            ping["ping_type"] == "metrics"
            and ping["start_time_local"] == ping["end_time_local"]
        ):
            ping["notes"].add(ZERO_LENGTH)

        # Find multiple pings with the same end_time
        if last_ping is not None:
            if ping["ping_type"] == "metrics" and last_ping["ping_type"] == "metrics":
                if ping["end_time_local"] == last_ping["end_time_local"]:
                    ping["notes"].add(DUPLICATE_TIME)
                    last_ping["notes"].add(DUPLICATE_TIME)
                else:
                    ping["notes"].add(NO_BASELINE)

        # Find missing or duplicate seq numbers
        last_of_same_type = last_by_type.get(ping["ping_type"])
        if last_of_same_type is not None:
            if int(last_of_same_type["seq"]) + 1 != int(ping["seq"]):
                ping["notes"].add(MISSING_SEQ)
            elif int(last_of_same_type["seq"]) == int(ping["seq"]):
                ping["notes"].add(DUPLICATE_SEQ)
                last_of_same_type["notes"].add(DUPLICATE_SEQ)

        if ping["ping_type"] == "metrics":
            # Find metrics pings that are more than 24 hours after the last baseline ping
            last_baseline = last_by_type.get("baseline")
            if last_baseline is not None and ping["end_time_local"] > last_baseline[
                "end_time_local"
            ] + datetime.timedelta(days=1):
                ping["notes"].add(TOO_LATE)

            # Find metrics pings that are more than +/-1 hour from 4am
            if abs(
                ping["end_time_local"]
                - ping["end_time_local"].replace(hour=4, minute=0, second=0)
            ) > datetime.timedelta(hours=1):
                ping["notes"].add(FAR_FROM_4AM)

        # Add notes to the overall client stats
        for note in ping["notes"]:
            client_stats.setdefault(note, 0)
            client_stats[note] += 1

        last_ping = ping
        last_by_type[ping["ping_type"]] = ping

    # Add client stats to the overall stats
    for note in client_stats.keys():
        stats.setdefault(note, 0)
        stats[note] += 1

    return client_stats


def process_single_client(client_id, client_data, stats):
    """
    Process a single client, performing the analysis and writing out a plot.
    """
    if has_timezone_change(client_data):
        stats["changed_timezones"] += 1
        return {"changed_timezones": True}

    client_stats = find_issues(client_data, stats)

    client_data.sort(key=lambda x: x["start_time_local"])
    metrics_rows = organize_plot(x for x in client_data if x["ping_type"] == "metrics")
    baseline_rows = organize_plot(
        x for x in client_data if x["ping_type"] == "baseline"
    )

    plot_timeline(client_id, client_data, metrics_rows, baseline_rows)

    return client_stats


def analyse_by_day(data):
    """
    Find the "issues" notated in the `notes` field on each ping and generate
    a graph of their frequencies over time.
    """
    data_by_day = {}
    for ping in data:
        if ping["ping_type"] == "metrics":
            day = ping["end_time_local"].replace(hour=0, minute=0, second=0)
            data_by_day.setdefault(day, {})
            day_data = data_by_day[day]
            day_data.setdefault("total", 0)
            day_data["total"] += 1
            for note in ping["notes"]:
                day_data.setdefault(note, 0)
                day_data[note] += 1
            for i, fix in enumerate(FIXES):
                if ping["version_date"] is not None and ping["version_date"] >= fix[1]:
                    fix_id = f"fix{i}"
                    day_data.setdefault(fix_id, 0)
                    day_data[fix_id] += 1

    # Trim the first and last couple of days, since they aren't meaningful
    data_by_day = sorted(list(data_by_day.items()))[2:-2]

    return data_by_day


def plot_summary(data_by_day, output_filename="summary.svg"):
    """
    Plot the summary of issues by day.
    """
    dates = [x[0] for x in data_by_day]

    plt.figure(figsize=(20, 20))
    plt.subplot(211)
    plt.title("Frequency of notes by day")
    for note in range(1, MAX_NOTES):
        note_values = [x[1].get(note, 0) / float(x[1]["total"]) for x in data_by_day]
        plt.plot(dates, note_values, label=NOTE_SUMMARIES[note])
    plt.legend()
    plt.grid()

    plt.subplot(212)
    plt.title("Uptake of fixes by day")
    for i, fix in enumerate(FIXES):
        fix_values = [
            x[1].get(f"fix{i}", 0) / float(x[1]["total"]) for x in data_by_day
        ]
        plt.plot(dates, fix_values, label=fix[0])
    plt.legend()
    plt.grid()

    plt.savefig(output_filename)


def main(input, output):
    data = load_data(input)
    data = filter_data(data)
    annotate_data(data)
    data_by_client_id = sort_data_by_client_id(data)

    if not os.path.isdir(output):
        os.makedirs(output)
    os.chdir(output)

    stats = {
        "total_clients": len(data_by_client_id),
        "changed_timezones": 0,
    }
    client_stats = {}
    for i, (client_id, client_data) in enumerate(data_by_client_id.items()):
        print(f"Analysing client: {i}/{len(data_by_client_id)}", end="\r")
        client_stats[client_id] = process_single_client(client_id, client_data, stats)

    plot_summary(analyse_by_day(data))

    print(stats)


if __name__ == "__main__":
    # Parse commandline arguments
    parser = argparse.ArgumentParser("Analyse patterns in baseline and metrics pings")
    parser.add_argument("input", nargs=1, help="The input dataset (in csv)")
    parser.add_argument("output", nargs=1, help="The output directory")
    args = parser.parse_args()
    input = args.input[0]
    output = args.output[0]
    main(input, output)
