import csv
from collections import defaultdict
from pathlib import Path
from sys import stderr

import httpx

from .converter import convert_group


def post(url: str, payload: dict):
    try:
        res = httpx.request("POST", url, json=payload)
        res.raise_for_status()
    except httpx.HTTPStatusError as exc:
        raise SystemExit(
            f"POST {url} failed: {exc} {exc.response.status_code}\n{exc.response.content}"
        ) from exc


def post_measurements(base_url: str, csv_files: list[Path]):
    measurements_by_timestamp: dict[str, list[dict[str, str]]] = defaultdict(list)
    for csv_file in csv_files:
        with csv_file.open(newline="", encoding="utf-8") as handle:
            for row in csv.DictReader(handle):
                measurements_by_timestamp[row["keen.timestamp"]].append(row)

    timestamps = sorted(measurements_by_timestamp)
    total = len(timestamps)

    for index, timestamp in enumerate(timestamps, start=1):
        values = [
            {
                "value": float(row["value"]),
                "unit": row["unit"],
                "system": convert_group(row["group"]),
                "label": row["label"],
            }
            for row in measurements_by_timestamp[timestamp]
        ]
        if len(values) != 8:
            raise SystemExit(
                f"unexpected measurement count for {timestamp}: {len(values)}"
            )

        post(f"{base_url}/measurements", {"values": values, "monitored_at": timestamp})

        if index % 250 == 0 or index == total:
            print(f"seeded measurements: {index}/{total}", file=stderr)
