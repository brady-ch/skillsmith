#!/usr/bin/env python3
"""Migrate skill references: foo.md -> foo-english.md + foo-wenyan.md; update index.toml and routers."""
from __future__ import annotations

import importlib.util
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKILLS = ROOT / "skills"
DICT_PY = Path(__file__).with_name("wenyan_dict.py")

SKIP_FILES = {"reference-router.md", "english-alternative.md"}


def should_skip_md(path: Path) -> bool:
    name = path.name
    if name in SKIP_FILES:
        return True
    if name.endswith("-english.md") or name.endswith("-wenyan.md"):
        return True
    return False


def skill_name(ref_dir: Path) -> str:
    return ref_dir.parent.name


def load_bodies() -> dict[str, str]:
    spec = importlib.util.spec_from_file_location("wenyan_dict", DICT_PY)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {DICT_PY}")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    d = getattr(mod, "d", None)
    if not isinstance(d, dict):
        raise RuntimeError("wenyan_dict.py must define dict `d`")
    return {str(k): str(v) for k, v in d.items()}


def key_for(skill: str, stem: str) -> str:
    return f"{skill}/{stem}"


def replace_router_links(text: str) -> str:
    """Point backticked *.md routes at *-wenyan.md (except router and already suffixed)."""

    def sub(m: re.Match) -> str:
        fname = m.group(1)
        if fname.endswith("-wenyan.md") or fname.endswith("-english.md"):
            return m.group(0)
        if fname == "reference-router.md":
            return m.group(0)
        base = fname.removesuffix(".md")
        return f"`{base}-wenyan.md`"

    return re.sub(r"`([a-z0-9][a-z0-9.-]*\.md)`", sub, text)


def english_block(filename: str, stem: str, priority: int) -> str:
    return f"""[[references]]
file = "{filename}"

[references.metadata.trigger]
summary = "Human-readable English mirror for {stem} (not for default agent routing)."
intent_tags = ["english", "human", "companion"]
when_to_use = [
  "Read the English companion for this topic.",
]
skill_role = "meta"
order_weight = 99

[references.metadata.objective]
summary = "Provide the English text for human readers and editors."

[references.metadata.output]
summary = "Return the same structure as the Wenyan agent slice, in English."

[references.metadata.navigation]
summary = "Human companion only; loses intent tie-breaks vs Wenyan slices."
priority = {priority}
"""


def patch_index(ref_dir: Path, raw: str) -> str:
    """Point file = entries at *-wenyan.md; append *-english.md entries once."""
    lines = raw.splitlines(keepends=True)
    out: list[str] = []
    for line in lines:
        stripped = line.strip()
        m = re.match(r'^file = "([a-z0-9][a-z0-9.-]*)\.md"\s*$', stripped) if stripped else None
        if m:
            stem = m.group(1)
            if not stem.endswith("-wenyan") and not stem.endswith("-english"):
                if (ref_dir / f"{stem}-wenyan.md").is_file():
                    line = line.replace(f'"{stem}.md"', f'"{stem}-wenyan.md"')
        out.append(line)
    text = "".join(out).rstrip() + "\n"

    indexed = set(re.findall(r'file = "([^"]+)"', text))
    extras: list[str] = []
    for eng in sorted(ref_dir.glob("*-english.md")):
        fn = eng.name
        if fn in indexed:
            continue
        st = eng.stem.removesuffix("-english")
        pri = 1010
        wm = re.search(
            rf'file = "{re.escape(st)}-wenyan\.md"[\s\S]*?\[references\.metadata\.navigation\][\s\S]*?priority = (\d+)',
            text,
        )
        if wm:
            pri = 1000 + int(wm.group(1))
        extras.append(english_block(fn, st, pri))
        indexed.add(fn)

    if extras:
        text = text.rstrip() + "\n" + "\n".join(extras) + "\n"
    return text


def main() -> None:
    dry = "--dry-run" in sys.argv
    bodies = load_bodies()
    missing: list[str] = []

    for ref_dir in sorted(SKILLS.glob("*/references")):
        if not ref_dir.is_dir():
            continue
        sk = skill_name(ref_dir)
        for md in sorted(ref_dir.glob("*.md")):
            if should_skip_md(md):
                continue
            stem = md.stem
            k = key_for(sk, stem)
            if k not in bodies:
                missing.append(k)
                continue
            eng_target = ref_dir / f"{stem}-english.md"
            wen_target = ref_dir / f"{stem}-wenyan.md"
            if dry:
                print(f"would migrate {md} -> {eng_target.name} + {wen_target.name}")
                continue
            eng_target.write_text(md.read_text(encoding="utf-8"), encoding="utf-8")
            wen_target.write_text(bodies[k].strip() + "\n", encoding="utf-8")
            md.unlink()

    if missing:
        print("Missing wenyan bodies for:", file=sys.stderr)
        for m in sorted(missing):
            print(f"  {m}", file=sys.stderr)
        sys.exit(1)

    if dry:
        return

    for ref_dir in sorted(SKILLS.glob("*/references")):
        index_path = ref_dir / "index.toml"
        if not index_path.is_file():
            continue
        raw = index_path.read_text(encoding="utf-8")
        new_raw = patch_index(ref_dir, raw)
        if new_raw != raw:
            index_path.write_text(new_raw, encoding="utf-8")

        router = ref_dir / "reference-router.md"
        if router.is_file():
            rtxt = router.read_text(encoding="utf-8")
            rnew = replace_router_links(rtxt)
            if rnew != rtxt:
                router.write_text(rnew, encoding="utf-8")


if __name__ == "__main__":
    if not DICT_PY.is_file():
        print(f"Missing {DICT_PY}", file=sys.stderr)
        sys.exit(1)
    main()
