# Wenyan agent bodies: key = "skill/stem" — run `python3 wenyan_dict.py > wenyan_bodies.json`
from __future__ import annotations

d: dict[str, str] = {}


def _a(k: str, v: str) -> None:
    d[k] = v.strip()


# --- api-contract-critic ---
_a(
    "api-contract-critic/contract-review",
    """# 契約察驗

## 觸發
- 審請求、應答、言行擔保、錯誤模型。

## 旨
- 見含糊、破壞行徑、闕相容之諾。

## 出
- 端點或函數之瑕、級序相容險、必澄清處、險與濟。

## 序
- 先載荷 schema 與必填
- 次缺省與省略語義
- 次錯誤面與態碼映射
- 次冪等、分页、序之諾""",
)

_a(
    "api-contract-critic/compatibility-matrix",
    """# 相容矩陣

## 用
- 類 API 變更：相容、有條、抑或破壞。

## 旨
- 按衝擊分級，示演進之路、遷移責任。

## 出
- 判詞、放行戒條、退路之期許。""",
)

# --- commit-after-tested-change ---
_a(
    "commit-after-tested-change/commit-cadence",
    """# Commit 之節

- 成且驗乃可 commit。
- 先跑切要之測，後乃 staging。
- `git status` 與 diff 須合本 slice。
- 題用短祈使，指名所驗之片。
- 測敗則修，勿以敗者為 checkpoint。
- 片未充、尚探索，則勿 commit。
- 多片不相涉，各 commit，勿捆無關之務。""",
)

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

# --- migration-guardian ---
_a(
    "migration-guardian/phased-plan",
    """# 階段遷移

## 用
- 分段推、橋相容、序割接。

## 旨
- 可逆之序、務保服續。

## 出
- 階序、相容之求、觀測之查。""",
)

_a(
    "migration-guardian/rollback-operations",
    """# 回滾操演

## 用
- 備回滾、敗象早識、守操演。

## 旨
- 敗如何覺、如何安全反轉。

## 出
- 閾、指令序、回滾後之驗。""",
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

# --- repo-scout ---
_a(
    "repo-scout/repo-briefing",
    """# 庫略

## 用
- 初識庫、出實施略。

## 旨
- 狀要、構要、變點。

## 出
- 現狀撮、方向、一顯險。""",
)

_a(
    "repo-scout/risk-triage",
    """# 險序

## 用
- 序險、塞點、濟務之先。

## 旨
- 以重排序、化為可執之步。

## 出
- 級序所見、今何礙、證法。""",
)

# --- test-determinism ---
_a(
    "test-determinism/nondeterminism-catalog",
    """# 無定類譜

## 用
- 測飄、時倚、異步競、CI 偶败。

## 旨
- 依 Fowler 類分之、指癥結。

## 出
- 似因之類、修法、何時升契約測。""",
)

_a(
    "test-determinism/isolation-and-parallelism",
    """# 隔與並

## 用
- 序倚敗、共夾、並跑壞測。

## 旨
- 減共有、並險、勿掩真敗。

## 出
- 隔或並之策、CI 守策之注。""",
)

# --- test-suite-design ---
_a(
    "test-suite-design/suite-levels-and-shape",
    """# 測層之形

## 用
- 單、集、端混；塔與杯；CI 費。

## 旨
- 明層喻、權衡白、指層比。

## 出
- 所荐層混、卻一途、API 險則指契約察。""",
)

_a(
    "test-suite-design/tdd-canon-and-loop",
    """# TDD 正典

## 用
- 納 TDD、紅綠、測先流。

## 旨
- 本諸 Beck，勿混極覆蓋。

## 出
- 工序、一戒、層與決定譜。""",
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

# --- behavioral-pattern-architect ---
_a(
    "behavioral-pattern-architect/pattern-selection",
    """# 行為術 快路由

## 圖
- 佇、誌、撤、重試：Command
- 串試至受：Chain of Responsibility
- 內聯訂者瞻主：Observer
- 題異步扇出：Publish-Subscribe
- 換等价策：Strategy
- 隨相變行：State
- 骨架定、鉤少變：Template Method
- 穩結上增算：Visitor
- 小程序反覆解：Interpreter
- 僚協歸中：Mediator
- 存復內相：Memento
- 空無為省 null：Null Object
- 商規組合：Specification
- 專家共黑板：Blackboard

## 淆
- Strategy 主外選；State 自模變
- Observer 內直訂；Pub-Sub 中介題
- Command 包行；Chain 委遞
- Mediator 規聚；Observer 廣播
- Template 骨不變；Strategy 全換
- Visitor 節上操作；Interpreter 樹即語

## 訣
- 先 Strategy、Observer、Command
- 組合先於擬繼
- 直引先於中介（非拓撲故）""",
)

_a(
    "behavioral-pattern-architect/behavioral-pattern-overview",
    """# 行為術 綱

## 義
- 物件協、職責分、演變柔。

## 族
- 行為分派、狀與流、通与協。詳各篇。""",
)

_a(
    "behavioral-pattern-architect/chain-of-responsibility",
    """# 責任鏈

- 請求沿鏈、直至某環受。
- 耦發者与受者、可動鏈序。
- 毋與中介混：鏈無中央脑。""",
)

_a(
    "behavioral-pattern-architect/command",
    """# 指令

- 請求封 객체、可撤、佇、誌。
- 發者与執者解耦。
- 宏命令可組細令。""",
)

_a(
    "behavioral-pattern-architect/observer",
    """# 觀者

- 主題變則眾觀者悉。
- 推模型、松耦一對多。
- DOM 等景常用。""",
)

_a(
    "behavioral-pattern-architect/publish-subscribe",
    """# 發訂

- 中介題、生消不知彼此。
- 異步扇出、拓撲伸縮。
- 與 Observer：此有 broker。""",
)

_a(
    "behavioral-pattern-architect/strategy",
    """# 策略

- 筭族封、可互換、運行選。
- 代繼承分支。
- 與 State：外選策、非內相自遷。""",
)

_a(
    "behavioral-pattern-architect/state",
    """# 狀態

- 相變則行為遷、狀物件代 if。
- 與 Strategy：內模遷、非主呼叫換。""",
)

_a(
    "behavioral-pattern-architect/template-method",
    """# 範法

- 骨不變子覆鉤。
- 控反轉、固序流。
- 與 Strategy：骨不變 vs 全筭換。""",
)

_a(
    "behavioral-pattern-architect/interpreter",
    """# 解器

- 語法例、樹遍求值。
- 文法增易、複合大則臃。""",
)

_a(
    "behavioral-pattern-architect/iterator",
    """# 迭器

- 序訪元、毋暴集內。
- 多遍法、可遲。""",
)

_a(
    "behavioral-pattern-architect/visitor",
    """# 訪者

- 新算舊結、雙分便利。
- 結穩算繁、增類煩。""",
)

_a(
    "behavioral-pattern-architect/mediator",
    """# 中介

- 僚不直引、規归一介。
- 星形、簡對象、介或臃。""",
)

_a(
    "behavioral-pattern-architect/memento",
    """# 備忘

- 存復內相毋違封。
- 快照可深可淺。""",
)

_a(
    "behavioral-pattern-architect/null-object",
    """# 空物件

- 空行代 null、省判。
- 需穩無所為語義。""",
)

_a(
    "behavioral-pattern-architect/specification",
    """# 規約

- 條可組與或非、商規顯式。
- 與 Strategy：規篩非筭。""",
)

_a(
    "behavioral-pattern-architect/blackboard",
    """# 黑板

- 專家觀黑板、機會協解。
- 不確、多域知、控難。""",
)

# --- creational-pattern-architect ---
_a(
    "creational-pattern-architect/pattern-selection",
    """# 造化術 快路由

## 圖
- 一族相關：Abstract Factory
- 創者鉤延子類實：Factory Method
- 繁物步裝：Builder
- 例克 runtime 克：Prototype
- 協調唯一：Singleton
- 根注入賴：Dependency Injection
- 首用方建：Lazy Initialization
- 短壽貴反池：Object Pool

## 淆
- 族 vs 單產：抽象工廠 vs 工廠法
- 步裝 vs 族：Builder vs Abstract Factory
- 克 vs 鉤：Prototype vs Factory Method
- 注 vs 廠：DI vs Factory
- 遲 vs 單：Lazy vs Singleton
- 池 vs 遲：Pool vs Lazy

## 訣
- 先 Builder、Factory Method、DI
- 注勝隱造
- 明廠勝單（非真需唯一調度）
- 池為能、非風格""",
)

_a(
    "creational-pattern-architect/creational-pattern-overview",
    """# 造化術 綱

## 義
- 例如何誕、何藏、何組。

## 族
- 實抽象、步裝、複例、唯一、注、遲、池。詳各篇。""",
)

_a(
    "creational-pattern-architect/abstract-factory",
    """# 抽象工廠

- 一族相關互配套。
- 具體廠換族、客戶不識類名。""",
)

_a(
    "creational-pattern-architect/factory-method",
    """# 工廠法

- 創延子類、超類定鉤。
- 單產線勝一產族。""",
)

_a(
    "creational-pattern-architect/builder",
    """# 裝者

- 步裝繁物件、同序多表。
- 導演可選。""",
)

_a(
    "creational-pattern-architect/prototype",
    """# 原型

- 克已置例、免子類爆炸。
- 深克戒環。""",
)

_a(
    "creational-pattern-architect/singleton",
    """# 單例

- 全域唯一協調、控入須慎。
- 測與並之仇、濫戒。""",
)

_a(
    "creational-pattern-architect/dependency-injection",
    """# 賴注

- 賴自外入、創不內藏。
- 便測、便換。""",
)

_a(
    "creational-pattern-architect/lazy-initialization",
    """# 遲建

- 首用方建、省啟。
- 與單不同：不一定唯一。""",
)

_a(
    "creational-pattern-architect/object-pool",
    """# 物池

- 短壽貴反用、減分配。
- 命與淨須約。""",
)

# --- structural-pattern-architect ---
_a(
    "structural-pattern-architect/pattern-selection",
    """# 結構術 快路由

## 圖
- 口異實協：Adapter
- 樹部同視：Composite
- 阻增裹：Decorator
- 繁面簡：Facade
- 遠代控：Proxy
- 抽與實獨：Bridge
- 享內外：Flyweight
- 型別標：Marker
- 句柄藏：Opaque pointer
- 域物聚：Aggregate
- 管線級：Pipes and Filters

## 淆
- Adapter 改口；Decorator 加責不改核
- Proxy 控達；Adapter 轉口
- Facade 簡子；Mediator 調僚

## 訣
- 能簡則 Facade、Adapter
- Flyweight 須眾享內""",
)

_a(
    "structural-pattern-architect/structural-pattern-overview",
    """# 結構術 綱

## 義
- 類與物如何拼、口如何統。

## 族
- 適配、合、飾、面、理、橋、享… 詳各篇。""",
)

_a(
    "structural-pattern-architect/adapter",
    """# 適配

- 舊類包新口、客不感。
- 與 Decor、Proxy 異。""",
)

_a(
    "structural-pattern-architect/bridge",
    """# 橋

- 抽與實各維獨變。
- 繼爆炸之濟。""",
)

_a(
    "structural-pattern-architect/composite",
    """# 合

- 樹部同型待、一視。
- 遍歸。""",
)

_a(
    "structural-pattern-architect/decorator",
    """# 飾

- 行責層裹、口不變。
- 與子類比：行組勝繼。""",
)

_a(
    "structural-pattern-architect/facade",
    """# 面

- 子繁歸一門、客簡。
- 不藏中介規。""",
)

_a(
    "structural-pattern-architect/flyweight",
    """# 享元

- 內外分、眾享內。
- 必能省內甚巨。""",
)

_a(
    "structural-pattern-architect/proxy",
    """# 理

- 代控達、遲建、權、遠。
- 口似實異。""",
)

_a(
    "structural-pattern-architect/marker-interface",
    """# 標介

- 空型契、語義型標。
- 元數據代亦可。""",
)

_a(
    "structural-pattern-architect/opaque-pointer",
    """# 朧指

- 實隱句後、曝不類。
- FFI 友。""",
)

_a(
    "structural-pattern-architect/aggregate",
    """# 聚

- 域內不變、根邊一致。
- DDD 物。""",
)

_a(
    "structural-pattern-architect/pipes-and-filters",
    """# 管濾

- 段純、 stdin/out 接。
- 並流易。""",
)

_a(
    "structural-pattern-architect/extensibility",
    """# 可展

- 開閉、插件、註冊。
- 定展點、毋改核。""",
)

# --- concurrency-pattern-architect ---
_a(
    "concurrency-pattern-architect/pattern-selection",
    """# 並術 快路由

## 圖
- 步寄主動對：Proactor
- 趣就緒反應：Reactor
- 互斥臨界：Lock / Monitor
- 讀寫割：Readers-Writer lock
- 屏障同步：Barrier
- 雙查建：Double-checked locking
- 觀而待：Guarded suspension / Balking
- 旗限流：Semaphore
- 工複用：Thread pool
- 域存線：Thread-local storage
- 表單步：Active object
- 合流：Join pattern
- 定時務：Scheduled task / Scheduler

## 淆
- Proactor 完後告；Reactor 緒告行
- Balking 不願等；Guarded 願等

## 訣
- 明模（反應、主動、排）乃擇
- 能消息則減共變""",
)

_a(
    "concurrency-pattern-architect/concurrency-pattern-overview",
    """# 並術 綱

## 義
- 任切、共護、步調、資限。

## 族
- 反應主動、同斥、同條、池… 詳各篇。""",
)

_a(
    "concurrency-pattern-architect/reactor",
    """# 反應器

- demux 緒、handler 短小非阻。
- 伸維在單線。""",
)

_a(
    "concurrency-pattern-architect/proactor",
    """# 主動器

- I/O 完調、適長耗非阻。
- OS 持能者佳。""",
)

_a(
    "concurrency-pattern-architect/lock",
    """# 鎖

- 臨界互斥、簡明死鎖戒。
- 粒度衡。""",
)

_a(
    "concurrency-pattern-architect/monitor",
    """# 監

- 併坊封；條變同。
- OO 語友。""",
)

_a(
    "concurrency-pattern-architect/readers-writer-lock",
    """# 讀寫鎖

- 多讀一寫、讀多益。
- 寫餓防。""",
)

_a(
    "concurrency-pattern-architect/double-checked-locking",
    """# 雙查

- 惰建、減鎖。
- 須內存序、易錯。""",
)

_a(
    "concurrency-pattern-architect/guarded-suspension",
    """# 守待

- 條未充則掛，充乃進。""",
)

_a(
    "concurrency-pattern-architect/balking",
    """# 卻步

- 時不濟則立返、毋等。""",
)

_a(
    "concurrency-pattern-architect/semaphore",
    """# 信号

- 計數坑、限並資。""",
)

_a(
    "concurrency-pattern-architect/barrier",
    """# 柵

- 眾相期一點、齊越。""",
)

_a(
    "concurrency-pattern-architect/thread-pool",
    """# 線工

- 工複用、削建燬。
- 隊滿策須。""",
)

_a(
    "concurrency-pattern-architect/thread-local-storage",
    """# 線域

- 每線獨私、毋鎖。
- 忌全域假。""",
)

_a(
    "concurrency-pattern-architect/active-object",
    """# 主動物

- 步隊序列、執單線。
- GUI 友。""",
)

_a(
    "concurrency-pattern-architect/join-pattern",
    """# 合式

- 告合流、綜多序果。""",
)

_a(
    "concurrency-pattern-architect/scheduled-task",
    """# 定務

- 後或週觸務。""",
)

_a(
    "concurrency-pattern-architect/scheduler",
    """# 調度

- 誰何時行、優先、饑。""",
)

# --- rust-patterns-architecture ---
_a(
    "rust-patterns-architecture/design-principles",
    """# Rust 則

## 源
- 術為溝具、須證權衡與題合。

## 畛
- 熟：慣。術：復發。反術：捷而險。

## Rust
- 明主與借、嚴別、型 API。
- 組合特勝擬繼。
- 命與別先於設。""",
)

_a(
    "rust-patterns-architecture/architecture-decisions",
    """# 構決

- 匣界、層、析、unsafe 殼。
- 組合先；unsafe 小、濃、可審。""",
)

_a(
    "rust-patterns-architecture/idiom-catalog",
    """# 熟目

- 主借形、簽宜人、就地遷。
- 術前語級取。""",
)

_a(
    "rust-patterns-architecture/api-ergonomics",
    """# API 宜

- 簽、泛界、呼端磨。
- 難誤用。""",
)

_a(
    "rust-patterns-architecture/common-patterns",
    """# 常術路

- 泛術在四技能；此庫 Rust 細。
- 路已術族乃本篇。""",
)

_a(
    "rust-patterns-architecture/behavioral-patterns",
    """# Rust 行為

- enum 判 trait dyn；RAII 行束。
- 族已定乃此。""",
)

_a(
    "rust-patterns-architecture/creational-patterns",
    """# Rust 造化

- builder、typestate、fold。
- 顯不變。""",
)

_a(
    "rust-patterns-architecture/structural-patterns",
    """# Rust 結構

- newtype、Deref 慎、trait 狀。""",
)

_a(
    "rust-patterns-architecture/boundary-safety",
    """# 界安

- unsafe 小、入檢、出守。
- 誌不變。""",
)

_a(
    "rust-patterns-architecture/custom-trait-bounds",
    """# 特界

- 擴外 crate 特、孤模塊便。""",
)

_a(
    "rust-patterns-architecture/functional-techniques",
    """# 函技

- iterator 鏈、組合子。
- 晰控制流。""",
)

_a(
    "rust-patterns-architecture/ffi-idioms",
    """# FFI 熟

- 命、對齊、錯跨、Drop。
- 朧指友。""",
)

_a(
    "rust-patterns-architecture/anti-patterns",
    """# 反術

- 神 enum、辮命、過 dyn、隱突變。
- 背型即背安。""",
)

_a(
    "rust-patterns-architecture/review-checklist",
    """# 閱檢

- 主、命、unsafe、err、併 API。
- 簽與行路之閱。""",
)

if __name__ == "__main__":
    import json

    print(json.dumps(d, ensure_ascii=False, indent=2))
