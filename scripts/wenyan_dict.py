# Wenyan agent bodies: key = "skill/stem" — run `python3 wenyan_dict.py > wenyan_bodies.json`
# Only skills shipped in catalog/catalog.toml locals are included.
from __future__ import annotations

d: dict[str, str] = {}


def _a(k: str, v: str) -> None:
    d[k] = v.strip()


# --- compression-skill-designer ---
_a(
    "compression-skill-designer/compression-mode-design",
    """# 壓縮技之則

## 用
- 欲省 token、立壓縮人格、啟用之律。

## 旨
- 觸發清、級次明、一字不改之禁、危則回落。

## 出
- 觸發句、壓縮層、保存律、安盾、例。

## 戒
- 涉契、安、法者不得瘦身失實。""",
)

# --- product-manager-challenger ---
_a(
    "product-manager-challenger/intake-and-challenge",
    """# 納詰

## 用
- 範弱、需糊、先施壓後行工。

## 旨
- 逼人、題、值、域、成標先明。

## 出
- 轢漏缺、未決之點、下一迫問。""",
)

_a(
    "product-manager-challenger/tracking-system",
    """# 蹤檢

## 用
- 庫內立 PM 檢表、執蹤、項次夾層。

## 旨
- 檢於庫、項有夾、狀可勾。

## 出
- 檔更、項態、闕執之節。""",
)

_a(
    "product-manager-challenger/item-routing",
    """# 項次路由

## 用
- 決細目入何檔於項夾之下。

## 旨
- 每類訊各歸一檔、毋積亂。

## 出
- 所取檔、其故、態之隨變。""",
)

# --- using-skillsmith ---
_a(
    "using-skillsmith/best-practices",
    """# 善例

## 用
- skillsmith 庫中：裝校、驗、裝技、啟規。

## 旨
- 根流、裝 targets、跑 validate、勿手掃目錄。

## 出
- 工作法、裝指、驗令、目錄戒。

## 先
- 先讀 `reference-router.md` 乃載他篇。""",
)

# --- software-architecture-architect ---
_a(
    "software-architecture-architect/architecture-decision-framing",
    """# 構策框

## 用
- 構決、質性、權衡敘。

## 旨
- 以制約與質性表方案、薦一途。

## 出
- 薦、卻一途、權衡之辭。""",
)

_a(
    "software-architecture-architect/decomposition-and-boundaries",
    """# 析界

## 用
- 模、包、服如何劈、賴何方。

## 旨
- 界置、賴向、耦之權。

## 出
- 荐界、卻一途、耦語。""",
)

_a(
    "software-architecture-architect/architecture-review-and-risks",
    """# 構評與險

## 用
- 審構、險析、敗式。

## 旨
- 见弱點、敗模、濟。

## 出
- 險、何礙、濟。""",
)

if __name__ == "__main__":
    import json

    print(json.dumps(d, ensure_ascii=False, indent=2))
