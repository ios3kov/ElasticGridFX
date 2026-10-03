"""Shared read-only installation decision contract. Never copies/replaces files.

Native frontends must collect fresh observations and execute/recheck separately.
Checksums bind bytes, not publisher authenticity; distribution trust is external.
"""
from dataclasses import dataclass
import re

TARGETS = {
    'macos-arm64': 'Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/FSTR FX/FSTR Stretch.plugin',
    'windows-x64': 'Adobe/Plug-ins/7.0/MediaCore/FSTR FX/FSTR Stretch.aex',
}
TRIPLES = {'macos-arm64': 'aarch64-apple-darwin', 'windows-x64': 'x86_64-pc-windows-msvc'}
BACKUP_ROOTS = {
    'macos-arm64': 'Library/Application Support/FSTR FX/Backups',
    'windows-x64': 'FSTR FX/Backups',
}
BUILD = re.compile(r'EGFX-[0-9a-f]{24}')
SHA = re.compile(r'[0-9a-f]{64}')
VERSION = re.compile(r'(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)')


@dataclass(frozen=True)
class Installed:
    # Collector normalizes location against the native platform's fixed root;
    # no path supplied by a package is ever a writable destination.
    relative_location: str
    build_id: str
    payload_sha256: str
    verified: bool


@dataclass(frozen=True)
class Observation:
    platform: str
    scan_complete: bool
    hosts_stopped: bool
    safe_destination: bool
    safe_backup_root: bool
    copies: tuple[Installed, ...]
    pending_transaction: bool = False


@dataclass(frozen=True)
class Candidate:
    platform: str
    target_triple: str
    version: str
    build_id: str
    payload_sha256: str
    clean_source: bool
    payload_verified: bool
    diagnostic: bool


@dataclass(frozen=True)
class Plan:
    operation: str
    target_relative: str
    backup_root_relative: str
    expected_old: Installed | None
    candidate: Candidate
    # This result never grants permission or represents actual installation.
    execution: str = 'NOT RUN'


def plan(candidate: Candidate, observation: Observation, *, expected_version: str,
         expected_build_id: str) -> Plan:
    """Fail closed; observations are evidence inputs, not execution switches."""
    flags = [candidate.clean_source, candidate.payload_verified, candidate.diagnostic,
             observation.scan_complete, observation.hosts_stopped, observation.safe_destination,
             observation.safe_backup_root, observation.pending_transaction]
    if any(type(flag) is not bool for flag in flags):
        raise ValueError('Ambiguous observation/candidate flags')
    if candidate.platform not in TARGETS or candidate.platform != observation.platform:
        raise ValueError('Wrong installation platform')
    if candidate.target_triple != TRIPLES[candidate.platform]:
        raise ValueError('Wrong native architecture/toolchain')
    if (not VERSION.fullmatch(candidate.version) or candidate.version != expected_version
            or not BUILD.fullmatch(candidate.build_id) or candidate.build_id != expected_build_id
            or not SHA.fullmatch(candidate.payload_sha256)):
        raise ValueError('Wrong candidate version/identity')
    if not candidate.clean_source or not candidate.payload_verified or candidate.diagnostic:
        raise ValueError('Unverified, dirty or diagnostic candidate')
    if not observation.scan_complete or not observation.hosts_stopped:
        raise ValueError('Close Adobe hosts and complete the duplicate scan')
    if not observation.safe_destination or not observation.safe_backup_root:
        raise ValueError('Destination/backup path safety not verified')
    if observation.pending_transaction:
        raise ValueError('Recover the pending transaction before installing')
    target, backups = TARGETS[candidate.platform], BACKUP_ROOTS[candidate.platform]
    if len(observation.copies) > 1:
        raise ValueError('Multiple copies preserved; resolve the conflict first')
    if not observation.copies:
        return Plan('CREATE_ONLY', target, backups, None, candidate)
    old = observation.copies[0]
    if (old.relative_location != target or old.verified is not True or not BUILD.fullmatch(old.build_id)
            or not SHA.fullmatch(old.payload_sha256)):
        raise ValueError('Unknown installed copy preserved')
    if old.build_id == candidate.build_id:
        if old.payload_sha256 != candidate.payload_sha256:
            raise ValueError('Installed identity has different bytes')
        return Plan('ALREADY_INSTALLED', target, backups, old, candidate)
    return Plan('BACKUP_THEN_REPLACE', target, backups, old, candidate)
