from json import dump as jd, load as jl

with (
    open("animations.json", "r") as jin
):
    animations = jl(jin)

with (
    open("animations_.json", "w") as jout
):
    jd(dict((x["Name"].replace("Idle", ""), {"frames": x["Frames"], "is_idle": x["Name"].startswith("Idle")}) for x in animations), jout, indent=2)
