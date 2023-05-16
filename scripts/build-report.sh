#!/bin/sh
#
# This script is used for building report.

# Stop on error.
set -e

# Check if the build directory exists.
if [ ! -d "build-report" ]; then
    mkdir "build-report"
fi

# Check if the report directory exists.
if [ ! -d "report" ]; then
    echo "The report directory does not exist."
    exit 1
fi

# Check if the report file exists.
if [ ! -f "report/report.tex" ]; then
    echo "The report file does not exist."
    exit 1
fi

# Copy the report file to the build directory.
cp "report/report.tex" "build-report/report.tex"

# Copy references to the build directory.
cp "report/references.bib" "build-report/references.bib"

# Copy images
mkdir -p "build-report/images"
cp -r "report/images" "build-report/images"

# Build tex file.
pdflatex -output-directory="build-report" "build-report/report.tex"

# Build bibliography.
biber --input-directory "build-report" --output-directory "build-report" "report"

# Build tex file.
pdflatex -output-directory="build-report" "build-report/report.tex"

# Copy the report file to the report directory.
cp "build-report/report.pdf" "report/report.pdf"
