from .args import parse_args
from .request import post, post_measurements

LABELS = [
    ("Heat Sink Temperature", "ヒートシンク温度"),
    ("Battery Voltage", "蓄電池電圧"),
    ("Array Voltage", "太陽光パネル出力電圧"),
    ("Target Voltage", "蓄電池目標電圧"),
    ("Total Kilowatt Hours", "総発電力量"),
    ("Total Amp Hours", "総発電流"),
    ("Array Current", "アレイ電流"),
    ("Charge Current", "充電電流"),
    ("Amp Hours", "積算電流"),
    ("Kilowatt Hours", "積算電力量"),
]

SYSTEMS = [
    ("Battery", "蓄電池"),
    ("Array", "太陽光パネル"),
    ("WindTurbine", "風力タービン"),
    ("Controller", "充放電コントローラ"),
    # ("Counter", "積算値"),
    # ("Temperature", "温度"),
]

UNITS = [
    ("Wh", "ワット時"),
    ("kWh", "キロワット時"),
    ("W", "ワット"),
    ("kW", "キロワット"),
    ("V", "電圧値"),
    ("A", "電流値"),
    ("Ah", "アンペア時"),
    ("C", "セルシウス度"),
]


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)

    if args.post_master:
        for label, remark in LABELS:
            post(f"{args.base_url}/labels", {"label": label, "remark": remark})
        for system, remark in SYSTEMS:
            post(f"{args.base_url}/systems", {"system": system, "remark": remark})
        for unit, remark in UNITS:
            post(f"{args.base_url}/units", {"unit": unit, "remark": remark})

    post_measurements(args.base_url, args.csv_files)
    return 0
