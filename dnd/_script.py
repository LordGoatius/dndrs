import pandas as pd
from copy import copy
import re

df = pd.read_csv("dnd-spells.csv")
cols = ['name', 'classes', 'level', 'school', 'cast_time', 'range', 'duration', 'verbal', 'somatic', 'material', 'material_cost', 'description']

for (i, series) in df.iterrows():
    name: str = series['name']
    level = series['level']

    if level == 0:
        level = "Cantrip"
    else:
        level = "L" + str(level)

    school = series['school']
    classes= series['classes'].split(", ")
    time = series["cast_time"]
    range = series["range"]
    duration = series["duration"]

    comp_v = bool(series["verbal"])
    comp_s = bool(series["somatic"])
    comp_m = bool(series["material"])

    comp_s = str(comp_s).lower()
    comp_v = str(comp_v).lower()

    # either NaN or a string containing a number immediately before GP
    cost: str = series["material_cost"]
    cost_desc = copy(cost)

    if not comp_m:
        comp_m = "None"
        cost = "None"
    else:
        # parse cost
        l = cost.split('gp')
        if (len(l) == 1):
            cost = "None"
        else:
            cost = int(l[0].split(' ')[-2].replace(',', ''))
            cost = f"Some(Cost::GP({cost}))"

    if comp_m != "None":
        comp_m = f'Some(MaterialComponent {{ components: "{cost_desc}", cost: {cost}}})'
    else:
        comp_m = "None"

    name_lit = name.upper()
    name_lit = name_lit.replace(' ', '_')
    name_lit = name_lit.replace('/', '_')
    name_lit = name_lit.replace(':', '_')
    name_lit = name_lit.replace('-', '')
    name_lit = name_lit.replace('-','_')
    name_lit = name_lit.replace("'", "")
    name_lit = re.sub("_\(.*?\)", "",name_lit)

    name_lit = name_lit.replace("__", "_")

    desc = series["description"]
    #  $name:ident,
    #  $namestr:literal,
    #  $level:expr,
    #  $school:literal,
    #  $time:expr,
    #  $range:expr,
    #  $duration:expr,
    #  $verbal:literal,
    #  $som:literal,
    #  $mat:expr,
    #  $desc:literal,
    #  $effect:literal
    # print("define_spell! {")
    # print(f"    {name_lit},")
    # print(f'    "{name.replace('"', '\\\"')}",')
    # print(f"    {level},")
    # print(f"    {school},")
    # print(f'    "{time}",')
    # print(f'    "{range}",')
    # print(f'    "{duration}",')
    # print(f"    {comp_v},")
    # print(f"    {comp_s},")
    # print(f"    {comp_m},")
    # print(f'    r#"{desc}"#')
    # print("}")
    if "Druid" in classes:
        print(name_lit)
