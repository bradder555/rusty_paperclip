from json import dump as jd, load as jl

with (
    open("animations.json", "r") as jin
):
    animations = jl(jin)


idle = []
action = []

out = {
    "idle": idle,
    "action": action
}

for animation in animations:
    if "Idle" in animation["Name"]:
        animation = dict(animation) #clone
        animation["Name"] = animation["Name"].replace("Idle", "")
        idle.append(animation)
    else:
        action.append(animation)

with (
    open("animations_.json", "w") as jout
):
    jd (out, jout, indent=2)
