import os
import toml
from sys import path
from shutil import rmtree

CWD = "."
SPECS_FOLDER = f"{CWD}/.spec/specs/"
TICKET_FOLDER = f"{CWD}/.ticket/tickets/"

def walk_folder(walk_root):
    to_delete = []
    for root, dirs, files in os.walk(walk_root):
        for file in files:

            if file in ["ticket.toml", "spec.toml"]:
                entity = toml.load(f"{root}/{file}")
                #print(f"{entity}")
                if "deleted" in entity and entity["deleted"]:
                    print(f"{file}: deleted = true:\n{entity}\n")

                    to_delete.append(root)

    return to_delete

l1 = walk_folder(SPECS_FOLDER)
l2 = walk_folder(TICKET_FOLDER)

to_delete = list(l1) + l2

for entry in to_delete:
    print(f"Deleting {entry}")
    rmtree(entry)
