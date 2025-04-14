#!/usr/bin/python3
import sys
import subprocess
import os 

# Find the test files by the numeric prefix
def find_files_by_prefix(directory, prefix):
    matching_files = []
    for filename in os.listdir(directory):
        if filename.startswith(prefix):
            matching_files.append(filename)
    return matching_files[0] # Always return first file as prefix is unique

def prase_command():
    prefix = sys.argv[1]
    dir = "logo_examples"
    # Full path for logo test files
    in_file = "logo_examples/"+ find_files_by_prefix(dir, prefix)
    out_svg_file = "output.svg"
    out_png_file = "output.png"
    command1 = ["cargo", "run", "--", in_file, out_png_file ,"200", "200"]
    command2 = ["cargo", "run", "--", in_file, out_svg_file ,"200", "200"]
    result = subprocess.run(command1, capture_output=True, text=True)
    result = subprocess.run(command2, capture_output=True, text=True)
    # Print the error
    print("Output:", result.stdout)
    print("Error (if any):", result.stderr)
    print("Exit Code:", result.returncode)

def main():
    prase_command()

if "__main__" == __name__:
    main()