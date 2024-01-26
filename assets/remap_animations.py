from json import dump as jd, load as jl
from yaml import dump as yd

with (
    open("animations.json", "r") as jin
):
    animations = jl(jin)


idle = []
action = []

for animation in animations:
    name = animation.pop("Name")
    frames = animation.pop("Frames")

    animation["frames"] = [
        {
            "duration": x.get("dur", 0), 
            "info": {
                "column": x.get("col", 0), 
                "row": x.get("row", 0)
            }
        } for x in frames
    ]
    if "Idle" in name:
        name = name.replace("Idle", "")
        idle.append(animation)
    else:
        action.append(animation)
    animation["name"] = name

aout = {
    "idle": idle,
    "action": action
}

out = {
    "animations": aout,
    "sprite_sheet_info": {"columns": 27, "rows": 34}
}

with (
    open("animations.yaml", "w") as yf
):
    yd(out, yf)
