Create an interface skeletonization utility that strips implementation bodies and returns only structural information.

Requirements:
- expose class/type/function signatures and layout
- hide implementation detail by default
- support quick architecture mapping across files with minimal token cost
- document when to prefer skeletonized inspection before full source reads