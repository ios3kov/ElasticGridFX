#!/usr/bin/env python3
import pathlib
import sys

if len(sys.argv) != 4:
    raise SystemExit("usage: verify_build_identity.py BUILD_RS LIB_RS SHELL_CPP")

build = pathlib.Path(sys.argv[1]).read_text()
lib = pathlib.Path(sys.argv[2]).read_text()
shell = pathlib.Path(sys.argv[3]).read_text()

build_required = [
    "ELASTICGRID_BUILD_ID",
    "ELASTICGRID_GIT_COMMIT",
    "ELASTICGRID_GIT_STATE",
    "BuildIdentity.txt",
    "ElasticGridBuildID",
    "ElasticGridGitCommit",
    "ElasticGridGitState",
    '["status", "--porcelain", "--untracked-files=normal"]',
]
lib_required = [
    'env!("ELASTICGRID_BUILD_ID")',
    'env!("ELASTICGRID_GIT_COMMIT")',
    'env!("ELASTICGRID_GIT_STATE")',
    "log_build_identity_once();",
    "AEHotLoader_ImplementationBuildID",
    "AEHotLoader_ImplementationCommit",
    "AEHotLoader_ImplementationGitState",
    "Build ID:",
]
shell_required = [
    "AEHotLoader_ImplementationBuildID",
    "AEHotLoader_ImplementationCommit",
    "AEHotLoader_ImplementationGitState",
    "build_id=",
    "commit=",
    "git_state=",
]

for item in build_required:
    if item not in build:
        raise SystemExit(f"missing build identity source contract: {item}")
for item in lib_required:
    if item not in lib:
        raise SystemExit(f"missing runtime identity source contract: {item}")
for item in shell_required:
    if item not in shell:
        raise SystemExit(f"missing shell identity source contract: {item}")

print("build identity contract: PASS")
