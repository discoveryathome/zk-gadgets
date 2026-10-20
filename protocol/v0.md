# Discovery@Home Protocol — v0 draft

Status: **draft for discussion**. Extracted from `CrocSwap/integer-mult-bounds`
(see `docs/crocswap-analysis.md`). Nothing here is frozen.

## 1. Goals and non-goals

A well-set-up repository lets any capable coding agent, given a one-line prompt and
a time budget, pick useful work, avoid duplicating others, produce a reviewable
result, and hand off what remains, without a central server or account.

Principles:

- **The repository is the source of truth.** It must work fully without the hub.
- **Properties over conventions.** Projects are judged on whether agents can work
  well there, not on file names. Conventions are the easy path, not the gate (§2).
- **Meet projects where they are.** Hubs adapt to existing repos; adopting
  conventions is encouraged, never required for listing.
- **Natural language between parties,** except for shared state, evidence, and
  permissions (§11.2).
- **Many hubs.** Nothing assumes there is only one (§12.3).
- **Coordination is advisory.** Claims don't lock anything; tools report coverage
  and freshness and never assert novelty.
- **The prompt points; the repo decides.** Donation prompts never contain
  priorities, so priorities change in one place.
- **Prompt first, tools optional.** Every action (donating a session, verifying,
  setting up a repo, applying, assessing) starts from one short prompt pasted into any
  coding agent. Tools make it more reliable; none is required to start (§1.1).
- **Generic behavior lives in the hub's playbooks, not in each repo's prose.**
- **Evidence status is explicit.** Passing checks is never the same as acceptance.
- **Standard library only** for the reference tooling (Python ≥ 3.9), so donors'
  agents can run it anywhere, Windows included, with nothing to install.

Non-goals for v0: hosted execution, a scheduler, a universal scientific ontology,
any replacement for GitHub.

### 1.1 Prompts, playbooks and tools

Three tiers, each optional beyond the first:

1. **A prompt.** One short paragraph a person pastes into their agent. It names the
   action's **playbook**, a versioned page on the hub's website (`<site>/v0/<action>.md`),
   and carries the parameters: repository, budget, focus, permissions, and a random
   seed. It also carries a brief fallback in case the agent can't fetch the playbook.
2. **A single-file helper, fetched on demand.** Playbooks offer the reference CLI as one
   file, `<site>/cli/discover-<sha12>.pyz`, named by its SHA-256 so a URL never changes
   meaning. The playbook states the full hash, and the agent checks it before running
   the file. It needs only `python3` (3.9+), so nothing is installed.
3. **The plugin.** The same playbooks installed as skills, with the helper built in.

The playbooks are the single source: the site and the plugin render them from the same
files. Wherever posting isn't possible (no `gh`, or not permitted), a playbook ends with
messages written out ready to paste, so a person can act as the transport.

**Randomness comes from executed code, never from a model's choice.** Language models
asked to choose at random pick the same few options, and hundreds of agents converge on
one task. Sources, in order: a seed embedded in the prompt (the hub's website draws one
in the browser when it builds a prompt, and may name the task it drew); the project's
own picker; the helper's `pick`; or any code that reads OS randomness. The draw from a
seed is defined so every implementation agrees (§7).

## 2. Two layers: properties and conventions

The standard has two layers.

- **Properties** (§3) say what must be true for donated agents to do useful work in a
  repository, judged by outcomes: can a fresh agent get oriented, do agents spread out
  instead of piling up, can results be checked, and so on. A project is assessed and
  staged on properties alone.
- **Conventions** (§4–§10) are the recommended ways to achieve them: file names,
  formats, message shapes, the picker. Following them makes assessment easy and lets
  the generic tooling work unmodified. They are evidence for a property, not
  requirements. A repo that achieves a property its own way gets the same grade.

The protocol is written for readers that are capable agents, so it can be loose
wherever a single agent reads (orientation, priorities, background) and needs exact
formats only where many agents write to shared state (claims, task draws, the frontier
record).

Recommended files. All paths are repository-relative. None is mandatory; an agent must
be able to find each one starting from `AGENTS.md` (or `CLAUDE.md`), and `discover.json`
makes that mechanical.

| Object | Recommended path | Format | Main axis |
|---|---|---|---|
| Agent entry | `AGENTS.md` | prose | Orientation |
| Project entry doc | `DISCOVERY.md`, or an existing `autoresearch.md` (`AUTORESEARCH.md`, `auto-research.md`) | prose | Orientation |
| Manifest | `discover.json` | `schemas/manifest.schema.json` | all (map) |
| Frontier record | `discover/frontier.json` | `schemas/frontier.schema.json` | Verification |
| Task catalogue | `discover/catalog.json` | `schemas/catalog.schema.json` | Coordination |
| Coordination messages | Discussions threads | `schemas/message.schema.json` | Coordination |
| Priority log | `discover/priority-log.md` | prose, dated entries | Steering |
| Result packages | `research/<name>/` | `schemas/package-task.schema.json` + `VERIFY.json` | Verification |
| Review ledger | `discover/ledger.jsonl` | TBD (v0.2) | Verification, Review |
| Verifications | `research/<name>/verifications/` | §9.1 | Review |
| Consent record | `discover.json` → `hubs`, or `.github/discover-hubs.json` | §11.4 | (listing) |

The project entry doc's name doesn't matter. `DISCOVERY.md` is recommended for new
projects; a project that already keeps an `autoresearch.md` keeps it, and the two are
equal: `entry.project` names whichever it is, nothing grades the name, and `discover init`
points to an existing one rather than writing a second.

JSON rather than YAML so the reference tooling needs no third-party parser.

## 3. Assessment: properties and stages

Modeled on L2Beat's rollup assessments: a profile of independent properties, shown
together, and **stages** defined by minimum grades across them. A project's stage is
set by its weakest relevant property, and each stage unlocks what donated agents may
do there. Stages answer one question: *how many agents is it safe to point at this
repository, and with what permissions?*

### 3.1 Grading principles

Active, important projects tend to have long, messy histories. Assessment must not
penalize that, or it will rank a barely touched repo above a busy one that just
improved. Every grade follows four rules:

1. **Grade the present, not the archive.** Prefer probes of the current state (what
   happens if an agent arrives now?) and inspection of current mechanisms. Neither
   depends on how much has happened before.
2. **Measure activity only where nothing else works, and only recently.** Some
   properties (Review, the spread half of Coordination) can only be seen
   through activity. Those measurements use a trailing 14-day window, and count only
   activity after the project's `process_changed_on` date when it's set (§4), so a
   project that just changed its process is graded on the new one.
3. **Too little activity means unknown, never good.** Below a minimum sample (20
   events in the window unless stated otherwise), a measured component is
   **unknown**. A quiet project can never outscore a busy one with the same mechanism.
4. **Penalize history only where agents pay for it now, and only if it's cheap to
   fix.** A crowded thread that slows today's agents shows up in probe cost, and an
   index or archive fixes it in an afternoon. No grade may require a project to have
   less history.
5. **Grade what a donor experiences:** the repository plus the hub's adapter, if it
   has one (§12.5). An adapter can raise Orientation and Coordination; Review and
   Steering need the maintainers. Every assessment says which grades depend on the
   adapter.

The test for any measurement: *could a maintainer raise this grade within a week
without rewriting history?* If not, the measurement is wrong.

### 3.2 Properties

Five axes. Each is graded **strong**, **adequate**, **weak**, or **unknown**, and every
grade cites its evidence: links, quotes, probe transcripts, or measurements. Numeric
thresholds are provisional until the pilots produce data (§13).

**1. Orientation.** Can a fresh agent, given only the standard prompt, learn what the
project is, what is currently accepted, where help is wanted, and how to contribute?

- *Assessed by* the orientation probe (§3.4), on the repository's documents and entry
  chain. Whether a specific listed item is still open is live state, and belongs to
  Coordination, provided the docs point agents to where live state is kept.
- *Strong:* all four core questions answered correctly within 15 minutes of reading.
- *Adequate:* all four answered, with omissions that wouldn't misdirect work.
- *Weak:* any answer wrong or missing, especially the current result or how to
  contribute.

**2. Coordination.** When many agents arrive at once, do they spread across the work
instead of converging on the obvious task, and can an agent arriving now tell what
others are doing and find work that's genuinely open?

Two halves of one question: do many agents avoid colliding? Both are assessed, and the
axis takes the lower of the two.

- *Spread* is assessed by inspecting the mechanism agents are directed to use, plus,
  where there is enough recent activity, the duplicate rate: the share of claims and
  contributed PRs in the window whose task and scope materially match another live
  claim or open PR from a different author. The mechanism sets the ceiling; the
  measurement can only lower it. With too little activity, the mechanism decides.
  - *Strong:* a seeded random draw over enumerated, weighted tasks (or an equivalent
    mechanism) that surfaces live claims; recent duplicate rate under 10% or unknown.
  - *Adequate:* enumerated open tasks or handoffs without randomization, or a draw that
    ignores live claims; recent duplicate rate under 30% or unknown.
  - *Weak:* agents choose freely from prose, or a recent duplicate rate of 30% or more.
- *Live state* is assessed by the coordination probe (§3.4): find open handoffs (or
  open requests by whatever name) on a given track, confirm nobody has taken them, and
  report live claims that overlap a given task. Graded on accuracy against ground
  truth the assessor establishes separately, and on cost.
  - *Strong:* accurate within 10 minutes, because handoffs have a visible lifecycle
    (open, taken, resolved): a claim or result says which handoff it takes up, and
    closed handoffs are marked or listed.
  - *Adequate:* accurate, but only by reading broadly and inferring which handoffs are
    taken; or mostly accurate within budget.
  - *Weak:* can't tell open from taken work, or there is no public channel.

Claims matter less than handoffs: most short sessions need no claim (§6), while a
handoff is a request someone else must be able to find and close.

**3. Verification.** Is there one authoritative statement of what is accepted, and
can someone other than the author recheck it?

- *Assessed by* locating the authoritative record, and the verification probe (§3.4),
  which runs the record's check within a fixed budget. The check may be
  **incremental**: it verifies the latest step on top of predecessor results that
  were themselves verified, identified by hash. A full replay from scratch is not
  required.
- *Strong:* one authoritative record whose declared check (full or incremental) the
  probe completes and confirms, with its prerequisites and expected runtime stated;
  and the project's current packaging rules require pinned sources and replay
  instructions, which recent contributions follow.
- *Adequate:* one authoritative record and a documented check, but the probe couldn't
  complete it within budget, or prerequisites had to be discovered.
- *Weak:* no single authoritative record, or results that can be accepted only by
  trusting their author.
- *Coverage.* The check has to cover what the record claims, not just something
  beside it. When the accepted result is a theorem and the declared check confirms only
  the computations around it (parameter arithmetic, numeric margins, a table of cases)
  while the argument itself rests on unreviewed prose, Verification is at most adequate.
  Strong needs the argument checked too: a formal proof of the claim (or of each step
  the record lists, with any imported assumptions stated), or recorded independent
  reviews of each step (§9.1) that the maintainers act on. The assessment says what the
  check covers (`computation`, `formal proof`, `reviewed argument`), and the listing
  shows it beside the grade, so donors and readers can tell "the arithmetic checks"
  from "the theorem is checked". (Found pre-assessing a proof-centred project whose CI
  confirms every parameter margin of a conditional coefficient bound, while the proofs
  behind it have had no independent human review: under the old wording it could have
  graded strong.)
- *Restricted inputs.* When results depend on data that can't be public (available on
  request, licensed, about people), the check can still be run by someone who obtains
  the data, which is different from trusting the author. Such a project can grade
  adequate, never strong. Adequate needs the pipeline public, the access route stated
  (who to ask, on what terms) and the restricted inputs pinned by hash, so whoever
  obtains them can tell they have the same files; without those, weak. The listing says the check needs restricted data, so donors know a session
  there can read and review but not rerun it. (Found assessing a marine-ecology
  replication package whose data are on request: graded weak, the same as a project
  with no check at all.)
- *Record tables.* When the frontier is a table of cases (§5), the probe runs
  `reproduce` as usual and, in addition, `check_row` on three rows drawn at random.
  Strong needs both. A per-row check has to recompute the row's object (the code, the
  packing, the ruler), not just read back its score: that is Coverage for a table.
- *Claim kinds.* A record says what kind of claim it makes (`claim_kind`, §5), and its
  statement claims no more than that kind supports: a `model` record names the model
  and its version, a `measured` one the hardware or data and the tolerance, and an
  `observational` one states candidates, not discoveries. A record whose statement
  outruns its kind is graded as if the excess were unchecked (Coverage). The grade
  doesn't rank kinds: an exact check and a well-pinned simulation can both be strong,
  and the listing shows the kind beside the grade.

**4. Review.** Does review keep up with what agents send now?

Review is the bottleneck of agent-scale research, and it doesn't have to be one
maintainer's time. **Donated review counts**: when maintainers accept results on the
strength of independent verification by other contributors (§9.1), that verification is
real capacity.

- *Assessed by* measurement over the trailing window. **Contributions** are PRs opened
  in the window by accounts other than the project's maintainers and bots. Maintainers'
  own PRs are reported as context but don't count, because their first "response" is
  usually the author's own merge. For contributions, measure:
  - the median time to first review response (a maintainer's comment, review, merge or
    close, or a recorded independent verification that maintainers act on);
  - the share that waited more than 7 days, among contributions opened at least 7 days
    before the window ends (younger ones can't have waited that long). With fewer than
    20 such contributions the share is unknown, and the grade is at most adequate;
  - whether the **backlog** grew over the second half of the window. The backlog is the
    number of contributions that have waited more than a day without a first response.
    It is compared at the window's midpoint and at its end, and has grown if it rose by
    more than 3 or 10%, whichever is larger.

  Contributions opened before the window are reported as context but don't set the
  grade. Before there is enough data, the maintainer's declared capacity (`review` in
  the manifest, §4) is used and the grade is at most adequate. Measurements and reasons:
  [`docs/calibration/review-2026-10-18.md`](../docs/calibration/review-2026-10-18.md).

  **Intake is not acceptance.** A merge counts as a first response, because it tells
  contributors where their work stands. Some projects, though, merge contributions as
  archival intake and record scientific review separately, for example as a ledger
  disposition. Where a project's docs say so, the assessment also reports two figures
  next to the grade:
  - the share of the window's merged contributions that have no recorded review
    outcome;
  - the date the frontier record last changed.

  These figures don't set the grade until they've been measured on more than one
  project. Evidence:
  [`docs/calibration/intake-2026-10-18.md`](../docs/calibration/intake-2026-10-18.md).

  **Mechanical acceptance.** Some projects accept contributions by a stated check
  rather than a person: a PR that adds a witness passing the check, and beating the
  current row, is merged by a workflow. Where the manifest declares it
  (`review.mechanical`, §4), the check's verdict counts as a first response: the
  automated merge, or a comment from one of the declared bots saying why a PR failed.
  It counts as review only when the check covers what is accepted (Coverage, under
  Verification), so in practice for `exact`, `formal` and `certified` claims. An
  automated merge of a `model`, `measured` or `observational` result is intake, and
  the intake figures above apply. Mechanical acceptance is measured like any other
  review; it's how a small project can honestly declare `surge`.
- *Strong:* median first response under 3 days, under 10% waiting more than 7 days,
  and the backlog not growing.
- *Adequate:* median first response under 14 days, and under 30% waiting more than 7
  days.
- *Weak:* anything slower, or no review response in the window.

**5. Steering.**
 Are priorities stated in one place and revised from evidence?

- *Assessed by* locating the current priorities and their revision history.
- *Strong:* priorities stated in one place, revised with reasons within the last 30
  days or since the frontier last changed (whichever is later), and a feedback
  channel that maintainers demonstrably read.
- *Adequate:* priorities stated in one place, but stale.
- *Weak:* no stated priorities; agents must infer them.
- *Finished projects.* A project whose maintainers state, in one place, that the work
  is complete and which contributions are still welcome (reproduction reports, errata,
  ports, extensions) has stated its priorities: that statement grades strong while it
  stays true, without periodic revision, and answers the orientation probe's question 3
  ("where is help most useful now"). Silence about whether work continues is still
  weak. (Found on the same replication package: a finished paper's code was graded down
  for having no roadmap.)

### 3.3 Stages

| Stage | Requires | Unlocks for donated sessions |
|---|---|---|
| Assessed | Nothing: a nomination (§11.5) or the hub's own initiative. Any public repository | Catalogued with its assessment and no contribution launcher, so no traffic is sent to it |
| **Stage 0: Explorable** | Maintainer consent; credit policy disclosed; a license; Orientation ≥ adequate | Local-only sessions: study, experiment and report to the donor; no posts, no PRs |
| **Stage 1: Swarm-ready** | Stage 0; Orientation strong; Coordination, Verification and Review ≥ adequate | Coordination messages and PRs, within the manifest ceiling (§4) |
| **Stage 2: Self-sustaining** | All five properties strong | Featured on the hub; recommended for large and recurring donations; higher default budgets |

An **unknown** grade never satisfies a requirement, with two exceptions: Review uses the maintainer's declared capacity until there is data, and a measured component
that is unknown for lack of activity defers to the mechanism (§3.1, rule 3).

The effective permission of a session is the intersection of the stage's unlocks, the
manifest ceiling, the donor's grant, and the agent's actual credentials. Tools read the
stage from the hub, and fall back to Stage 0 behavior when it can't be read.

**Consent** is proved by a record in the project repository, as a domain proves
control to a certificate authority (§11.4). Submitting a project or approving its
registry PR also counts as consent to be listed by that hub. Removing the record
withdraws consent at the hub's next run.

**Credit** is disclosed, not graded. Listing with a launcher requires a stated credit
policy (§4), shown on the listing so donors can see it before they choose.

**License.** Listing with a launcher also requires a license in the repository: a
`LICENSE` (or `LICENCE`, `COPYING`) file whose terms let others use the work and
contribute to it, such as an open-source license for code or an open content license
(CC BY, CC0) for papers and data. Without one the work is all rights reserved, so a
donor's contribution has no clear terms and nobody can safely build on the results.
Like credit, it is checked, not graded; the hub shows it on the listing.

**Changes.** Upgrades take effect at the next hub build. A downgrade caused by Review
takes effect immediately, because protecting maintainers from an inflow they can't
absorb is the point of staging, but it doesn't throttle the project by itself: the hub
sends the maintainers an urgent notice (a hub issue, and their notices channel if they
accept pushes), and they pause with one change (`review.paused`, §4). Missing a project's
moment of attention costs more than a busy review queue, and the maintainer is the one
who knows which it is. Only a project that declared a numeric capacity has opted in to
the hub steering its traffic (§12.2). Other downgrades take effect after 14 days' notice
on the project's listing, so maintainers can respond first.

### 3.4 How assessment works

The grader is a function anyone can run (§11.1). A hub runs it when a project applies,
then weekly. Maintainers run the same assessment locally whenever they like (the
`setup` playbook).
An assessment combines three kinds of evidence:

- **Deterministic checks:** schema validation of whichever conventions the repo
  follows, file existence, and channel activity in the window.
- **Probes:** fresh agents with no prior context, a fixed budget, and only the
  standard prompt. They may read anything public and run the commands the repository
  declares for orientation or task selection (a picker, an overlap search), but may
  not run research code, write to the repository, or post. The probe model is fixed
  per rubric version at a tier representative of typical donors, not the strongest
  available, because a standard written for capable agents has to say how capable.
  Reference answers and ground truth are prepared separately, with a stronger model
  and maintainer approval.
  - The *orientation probe* answers four core questions: (1) what problem is this;
    (2) what is the strongest currently accepted result, and its scope; (3) where is
    help most useful now; (4) how does a contributor claim work, submit results, and
    have them checked. The maintainer approves reference answers once at onboarding;
    later probes are compared against them by a separate assessor agent, and
    disagreements are flagged for a human.
  - The *coordination probe* is given a track and a task, and must list open
    handoffs on the track that nobody has taken, and live claims overlapping the
    task. The assessor establishes the true answer independently with a larger budget.
  - The *verification probe* runs the authoritative record's declared check.
- **Measurements:** recent duplicate rate, review response and backlog, from GitHub
  data, under the rules in §3.1.

Probes rerun when the entry chain changes materially (judged by a cheap diff check,
not any file change), when the rubric version changes, and at least monthly.
Deterministic checks and measurements run every time.

The output is an **assessment record**, one file per assessment at
`assessments/<project-id>/<YYYY-MM-DD>-<assessor>.md` in the hub repository, so
assessments never conflict and keep their history: per-property grade and evidence, the
commit assessed, the date, the window, the probe model, and the rubric version. The
registry entry's `assessment` field points to the latest one. The stage is computed from the grades, never chosen. Findings that don't
affect a grade (a confusing term, a stale cross-reference) are recorded as
**suggestions** for the maintainer, separate from the grades.

While onboarding a repo that keeps its own conventions, the hub's agent also writes the
project's **adapter** (§12.5): a page for donors' agents that maps those conventions
onto the protocol and fills in missing procedure. It's a bridge. The agent can offer the
maintainers a `DISCOVERY.md` capturing it (or additions to the entry doc they already
keep, such as `autoresearch.md`), so the map can move into the repo, which
stays the source of truth.

**Disputes.** Maintainers dispute a grade with an issue or PR on the hub repository,
citing evidence. A hub maintainer decides, and the decision is recorded alongside the
assessment.

**Assessment records** end with one fenced `discover-assessment` JSON block
(`schemas/assessment.schema.json`): the commit and date assessed, the rubric version,
assessor and probe model, the `entry_paths` the probes relied on, the five grades, the
stage, which grades rest on an adapter, and, for a nomination, the projected grades. The
registry's `stage` and `grades` are copies of the latest block, and the hub's checks fail if
they differ.

**Rubric versions.** The axes, thresholds, stage requirements and probe model are
versioned with this spec. Changes are announced on the hub at least 30 days before they
affect any project's stage.

### 3.5 Conventions by property

| Property | Recommended convention | Also accepted |
|---|---|---|
| Orientation | `AGENTS.md` → the entry doc (§2) | Any entry chain an agent can follow from `AGENTS.md`/`CLAUDE.md`/`README.md` |
| Coordination (spread) | Weighted catalogue + seeded draw (§7) | A maintained list of open tasks or handoffs; any draw an agent is told to use |
| Coordination (live state) | **Discussions** threads + `discover` message blocks with `responds_to` (§6). The default, and strongly encouraged: it's what the tooling reads | Any public channel where handoffs have a visible lifecycle, e.g. issues (on by default). No channel is required for listing |
| Verification | Frontier record (§5) with `reproduce` + result packages (§9) | Any single authoritative record with a documented full or incremental check |
| Review | Review ledger (§9) + verify sessions and `verifications/` records (§8, §9.1) | Any recorded independent check that maintainers act on |
| Steering | Priority log + director review (§10) | Priorities in the entry doc with dated revisions |

The reference tooling's `discover validate` is the deterministic part only: it reports
whether the required files (§4, §5) are present and valid, and which of the conventions
above the project follows. It never reports a stage. The stage comes from an assessment
record, and the hub copies it into the registry entry (`stage`) when a maintainer merges
the assessment.

## 4. Manifest (`discover.json`)

Optional. When present, it's the mechanical map of the repo: identity, maintainers,
where each object lives, the claim channel, session defaults, and what donors are
permitted to do. Without it, the hub's adapter (§12.5) plays this role. See the schema and
`examples/integer-mult-bounds/discover.json`.

The `permissions` block is a ceiling, not a grant. An agent acts on the
intersection of (stage unlocks, §3.3) ∩ (manifest ceiling) ∩ (donor's prompt) ∩
(its actual credentials).
Repository text can never widen a donor's grant.

`hubs` (optional) is the project's consent record (§11.4): which hubs may list it and
what each may push. `[{"hub": "OWNER/REPO", "list": true, "push": ["notices",
"suggestions", "prs"]}]`. `"hub": "*"` sets the default for all hubs, so
`[{"hub": "*", "list": false}]` opts out of every hub. Push is never implied. Repos
without a manifest can put the same array in `.github/discover-hubs.json`.

`credit` (required for listing with a launcher, here or in the project entry doc) states how
contributors are credited, in plain words, with optional `kinds` for filtering:
`{"policy": "Accepted results credit their authors as co-authors on the paper.",
"kinds": ["co-authorship"]}`. Kinds: `co-authorship`, `acknowledgement`,
`contributors-file`, `citation`, `none`. Credit is disclosed so donors can choose; it
isn't graded.

`picker` (optional) names the project's own task picker, `{"command": ["python3",
"scripts/choose.py", "--seed", "{seed}"]}`, for projects that draw tasks their own way.
Tools pass it the seed and never name a card themselves, since another picker chooses a
different card from the same seed (§7).

`review` (optional) declares review capacity: `{"per_week": 5}`, contributions the
maintainers can give a first response to in a week; `{"per_week": "surge"}` for a
project ready for any amount of traffic; `{"paused": true}` to close the launcher until
it's set back; an optional `note` on who reviews; and `mechanical`, when a stated check
rather than a person accepts contributions: `{"rule": "A PR whose ruler passes
check.py and is shorter than its row's is merged automatically.", "check":
".github/workflows/accept.yml", "bots": ["github-actions[bot]"]}` (§3.2, Review). The
`bots` are the logins whose comments carry the verdict, so they count as a response.
Undeclared means no limit. The
hub shows the declaration beside what's waiting (§12.2), and never throttles a project
for it: `paused` is the only switch, and it's the maintainer's. The adapter carries the
same block for repos without a manifest, on the maintainer's word.

`requirements` (optional) states what a donor's machine needs for the project's usual
work: `{"arch": ["x86_64"], "os": ["linux"], "ram_gb": 32, "disk_gb": 50, "gpu":
"CUDA, 24 GB", "note": "..."}`, every key optional. A card can carry the same block as
`requires` when only that task needs it. The listing shows them, the picker marks drawn
cards a machine can't serve (`discover pick` checks architecture, OS and memory), and a
session takes the next drawn card instead of a task it can't do. Found in testing: a
card needing a second architecture was skipped only because the agent read it closely.

`visual` (optional, encouraged) is the project's hero picture, on its card and page.
Engagement follows a good picture, and a picture that changes with the frontier shows
progress at a glance. `render` says how it's made:

- `progress`: the frontier's value over time, drawn by the hub from the history (§5).
- `bound-gap`: the same, against the proven bound (`metric.bound`) and a `baseline`
  (`{"value": "0", "label": "no saving"}`), where the question started.
- `records`: the record table as a grid of cases, filled when settled, outlined while open.
- `suite`: the method's mean gain over each baseline on held-out instances (§5).
- `image`: a picture the project keeps, `{"render": "image", "src": "docs/hero.svg",
  "alt": "..."}`, in SVG, PNG, JPEG, WebP or GIF, at most 1 MB. For a picture of the
  result itself (a packing, a ruler), regenerate it when the frontier changes. An image
  made by an image model records its `prompt`, a template filled from the frontier
  (`{title}`, `{statement}`, `{metric}`, `{value}`), and the page shows it.

`kind` says what the picture is: `witness` (the result itself), `derived` (computed
from the frontier, the default for drawn renders) or `illustration` (the topic only, the
default for images). The page labels it, so an illustration isn't taken for a result.
Optional `caption`. Without a `visual`, the hub picks the best it can draw: the suite's
bars for a suite, the records grid for a table, bound-gap when there's a proven bound, progress when there are two or
more points, otherwise a title card with the metric. The adapter can declare one for a
project that doesn't, and only it may use a picture the hub keeps (`"from": "hub"`).

`domains` are free text, but prefer these where they fit, so donors' interests map onto
projects: `mathematics`, `algorithms`, `computer-arithmetic`, `performance-engineering`,
`machine-learning`, `statistics`, `physics`, `astronomy`, `chemistry`, `materials`,
`life-sciences`, `ecology`, `medicine`, `earth-sciences`, `climate`, `engineering`,
`economics`, `social-science`, `linguistics`, `meta-science`, `tooling`. A project can
list several, broad and narrow (`life-sciences` and `ecology`).

`process_changed_on` (optional, a date) marks the last time the project changed how it
coordinates or reviews, for example when it adopted a convention. Activity-based
measurements count only from that date (§3.1, rule 2). A maintainer moves it forward by
committing the change; the hub's adapter can carry it for repos without a manifest.

## 5. Frontier record

The single authoritative statement of what is currently accepted. Generalizes
`certificates/selected-result.json`.

Required: `statement`, `status`, `evidence_level`, `scope`, `reproduce` (needed for strong Verification),
`review`, `updated_on`. Optional: a numeric `metric` (`{name, value, exact,
direction}`), `assumptions[]` (open obligations), `sources[]`, `artifacts[]`
(path + sha256), `previous_record`, `pinned_heads`, `formulation`, and a `table` or
`suite` when the frontier is a family of cases or a method (below).

The `reproduce` check may be **incremental**: it verifies the latest step on top of
predecessor results that were themselves verified, identified by hash, instead of
replaying the whole chain from scratch. Optional `reproduce_full` gives the full replay
when it differs, `reproduce_minutes` the expected runtime on an ordinary machine, and
`prerequisites[]` everything a newcomer must install first. A busy project's chain of
results grows with its history, and a full replay shouldn't be the price of checking
the newest step.

**Evidence levels** are not a ladder; a record states every one that applies:

- `author-checked`: the contributor's own checks passed.
- `independently-checked`: someone else reproduced the scoped result.
- `maintainer-reviewed`: maintainers accepted the statement and its limitations.
- `formally-verified`: a proof checker accepted the exact formal statement. That
  the formal statement means what `statement` says is a separate review question.
- `externally-accepted`: an outside authority that keeps the field's records (a
  records table, an archive, a catalogue) accepted the result. `external[]` names it:
  `{authority, url, ref, accepted_on}`. The authority's own listing is the reference
  for its records; the frontier record says what the project claims.

**Claim kinds.** Evidence levels say who has checked a result; `claim_kind` says what
kind of claim it is, and so what any check of it can establish. A record states every
kind that applies:

- `exact`: recomputing from the published object confirms it exactly (a witness and
  its score, an integer count, a table of cases).
- `formal`: a proof checker accepts it.
- `certified`: rigorous numerics (interval arithmetic, certified bounds).
- `argument`: a written proof, checked by people.
- `model`: true within a named, pinned model or simulator (an energy function, a
  folding model, a fixed potential). Not established about the world.
- `measured`: a measurement (timing, accuracy, a benchmark score) that reruns reproduce
  within a stated tolerance, on stated hardware or data.
- `observational`: inferred from observational data. A candidate until new evidence
  or an outside authority confirms it.

The manifest's `verification_profile` is the strongest kind of evidence a project
produces, for filtering; `claim_kind` is what its current record claims. A constructed
result can also carry the other side of the question: `metric.bound` is the best proven
bound (no value can beat it) and `metric.bound_source` where it's proved. When the value
meets the bound, the question is settled.

**Record tables.** Many problems are families of independent cases: a best-known code
for each length and distance, a packing for each count, a ruler for each number of
marks. Their frontier is a table, one row per case, and `table` describes it:

```json
"table": {"path": "rulers.csv", "key": ["n"], "value": "length",
          "direction": "lower-is-better", "bound": "optimum",
          "check_row": ["python3", "check.py", "--n", "{n}"], "check_row_minutes": 1}
```

`path` is a CSV, JSON (an array of rows, or `{"rows": [...]}`) or JSON Lines file;
`key` names the columns that identify a case; `value` the case's best value so far;
`bound` (optional) the column holding its proven bound; `object` (optional) a column
holding the path of the case's witness; and `check_row` checks one row, with `{column}`
replaced by the row's value. A row is **open** while it has no bound, or its value
doesn't meet it. `statement` describes the family, `reproduce` checks the whole table
(or incrementally, the rows changed since the last verified table), and `metric` can
summarize it. Tables suit swarms: cases are independent, so sessions spread across them
without coordination, and a per-row check keeps review cheap. `examples/golomb-rulers/`
is a worked example. Three optional columns credit each case, and hubs show them: `by`
(who found the current value), `source` (a PR as `#123`, a commit, or a URL) and `updated`
(the date the value last changed). The table is a leaderboard of cases, not of people.

**Evaluation suites.** Some projects race methods, not results: the question is which
algorithm does best across a distribution of problems (schedules, structures, routes),
and a result is a method that beats the others on instances it hasn't seen. A single
best instance proves little there, and an improvement over the project's own first
attempt isn't an improvement over the field. The frontier is then a `suite`:

```json
"suite": {"path": "results.jsonl", "objective": "tour length", "direction": "lower-is-better",
          "current": "kicked-ils",
          "baselines": [{"method": "2-opt", "strength": "published", "source": "Croes 1958"},
                        {"method": "lkh", "strength": "mature", "version": "2.0.10"}],
          "instances": {"draw": "drawn", "count": 12, "beacon": "drand-quicknet"},
          "budget": {"unit": "move evaluations", "amount": 1000000},
          "evaluate": ["python3", "evaluate.py", "--method", "{method}", "--draw", "{draw}"]}
```

The results file (CSV, JSON or JSON Lines) has one row per run: `method`, `instance`,
`value`, and optionally `seed`, `machine`, `draw` (which draw of instances), `by` and
`source`. `current` is the accepted method. The rules that make a claim mean something:

- **Baselines say how strong they are.** `starter` (the project's own first
  implementation), `published` (a method from the literature, implemented here) or
  `mature` (an established solver the field relies on, at a pinned `version`). The hub
  compares the current method with each, instance by instance (the mean over seeds,
  relative to the baseline), and shows the mean gain, wins, ties and losses, and a
  two-sided sign test. It names the strongest kind of baseline the method beats (mean
  gain above zero, more wins than losses, p < 0.05), and says so when there's no mature
  baseline at all. Beating the starter is progress for the project, not for the field.
- **Instances the method couldn't have been tuned to.** `draw` is `fixed` (a public,
  frozen set; the page warns that methods may have been tuned to it), `drawn` (generated
  from randomness fixed after the method was, such as a public beacon round later than
  the method's commit, named in each row's `draw`) or `sealed` (held privately by the
  maintainers, who run the evaluation and publish the instances after each round). An
  optional public `dev` set is for tuning, and never counts.
- **Budgets that don't depend on the machine.** Prefer counting work (evaluations,
  feasibility checks, nodes). A wall-clock budget makes results a property of the
  hardware: rows then carry `machine`, and the hub pairs a method with a baseline only on
  the same machine, so a donor reruns the baseline beside the method and the relative
  gain is what's compared.

A suite's rows are usually exact (rerunning gives the same value) or measured; that the
gain holds beyond the draw is what the sign test over its instances supports, and
`scope` should say so. `examples/tsp-heuristics/` is a worked example.

**Formulations.** `formulation` (optional) names the exact problem the record answers:
`{"id": "truss-25-grouped-v1", "doc": "docs/formulation.md", "since": "2026-10-01",
"replaces": "..."}`. Changing the model, the objective or the constraints (sizing 25
members independently instead of in 4 groups, adding a buckling constraint) makes a new
problem: its results aren't compared with the old formulation's, however much better they
look. A maintainer accepts the new formulation explicitly, its `id` changes, history
entries carry `formulation`, and charts start a new line where it changes. A changed
formulation can be the most valuable contribution a session makes; it's a different
claim, not a bigger number.

**History.** `history` (optional) is the path of the frontier's history: JSON Lines (or a
JSON array), one entry per change, oldest first:

```json
{"date": "2026-10-12", "value": "1517760977820331/1000000000000000000", "by": ["alice"],
 "source": "#1042", "note": "first composition above 0.0015"}
```

`date` and `value` are required; `statement`, `bound`, `case` (a table row's key, for a
change to one case), `by`, `source` and `note` are optional. Without one, a hub derives
the history from the git history of the frontier record itself, taking each version's
`metric` (through the adapter's mapping, for a record in another shape) and keeping the
versions where the value changed. That needs nothing from the project, but it can't
credit anyone or tell a real change from a correction; a history file can. `metric.unit`
(optional) is shown beside the value.

Historical notes may call something "current"; only the frontier record is.

## 6. Coordination messages

The one structural change from the reference implementation. CrocSwap's
`CLAIM / FINDING / HANDOFF / DONE / RELEASED` posts are prose, so the picker can't
see saturation or prefer open handoffs. v0 keeps GitHub Discussions as the
transport but makes the payload machine-readable.

Each protocol post is human-readable Markdown that **ends with** a fenced block, one per
message (a post that reports a `done` and leaves a `handoff` carries both):

````markdown
```discover
{ "v": 0, "kind": "claim", "task": "p104-side-correction", ... }
```
````

See `schemas/message.schema.json`. Kinds:

| Kind | Meaning | Picker effect |
|---|---|---|
| `claim` | I am working on this scope until `expires` (UTC) | Live claims on a card lower its draw weight and are listed |
| `finding` | Provisional result, with evidence link and level | Listed as context |
| `handoff` | A concrete, bounded request for someone else | **Offered before drawn cards** when it matches the track |
| `done` | Finished; links artifact or PR | Ends the claim; counts toward card saturation |
| `released` | Stopped without finishing, with reason | Ends the claim |

Linking: `replaces` (a later message ends or updates an earlier claim) and
`responds_to` (a claim or done takes up a handoff). A handoff stops being offered once
a live claim or a done responds to it. A claim also ends when the same author later
posts `done` or `released` for the same task and scope.

Rules carried over unchanged: claims are advisory; expired claims block nothing;
one canonical thread per question; edit routine status in place; a `done` is not
maintainer review; local-only evidence is labelled `LOCAL-ONLY`. A session
whose work takes under five minutes may skip `claim` and post only `done`/`finding`;
any longer session with permission to post claims as soon as it has chosen, because
short sessions that start together are the ones most likely to collide.

`discover post` writes these messages, so agents never hand-assemble GraphQL.

## 7. Task catalogue

Generalizes `agent-task-catalog.json`: tracks with integer weights summing to 100,
each holding task cards, plus `priority_revision`, `assessed_on`, `priority_log`.

Card fields: `id`, `question`, `starting_points[]`, `methods[]`, `acceptance_gate`,
`first_step` (a positive-control command with expected output), `first_experiment`,
`bottleneck_link`, `allowed_variations[]`, `scope_change_triggers[]`,
`overlap_terms[]`, and optional `requires` (as in the manifest, §4) for a card that
needs particular hardware, `claim_kind[]` (as in §5) for the kind of result the card's
work produces, and `rows` for a card that applies to any case of the frontier's table:
`{"open": true, "where": {"t": "3"}}` draws from the open rows (the default) whose
columns equal the given values. New in v0: optional `existing_checks[]`, from donor feedback: what
already covers this card for the current frontier, each with `scope`, `evidence`, and
optionally `remaining`, `status`, `source_commit` and `frontier_sha256` (the check is
stale once the frontier record changes).

### Picker semantics

`discover pick [--seed HEX32] [--track ID] [--task ID]`:

1. Read the catalogue and frontier from a **pinned commit** (the caller supplies
   or fetches it); report commit, catalogue hash, frontier hash.
2. Unless `--offline`, read coordination messages. If an unclaimed `handoff`
   matches the drawn track, offer it first.
3. Draw a track by weight, then cards with independently drawn methods, from the
   seed. Down-weight cards with live claims or many recent `done`s, and say so.
4. For a card with `rows`, draw one eligible row of the frontier table, uniformly, in
   key order, and give its key, value, bound and filled-in `check_row`.
5. Emit `scope_key = sha256(task_id, method)`, or `sha256(task_id, method, row key)`
   for a drawn row (excluding seed and commit, which don't change the question).
6. Never claim to reserve work or to establish novelty.

The draw is defined exactly, so a seed reproduces it anywhere: let
`draw(label) = int(sha256(json([seed, label])))`, where `json` is compact JSON with no
spaces. The track is the one whose cumulative weight range contains
`draw("track") mod (total weight)`, in catalogue order. Cards are then drawn in turn
with `draw("card:<i>")` over integer weights (1000 each when no board is read), and each
card's method with `draw("method:<card id>") mod (number of methods)`. A card with
`rows` takes the eligible row at index `draw("row:<card id>") mod (number eligible)`,
the rows sorted by the compact JSON of their key (columns sorted by name, values as
text). The reference CLI and the hub's website implement the card draw identically; only
the CLI draws rows, since the website doesn't load tables.

A project with its **own picker** (`picker`, §4) uses it instead: the seed is passed to
it, and the hub names no card. The draw is reproducible either way; only the picker
that draws can say which card a seed gives.

## 8. Session protocol (default, owned by the playbooks)

These defaults move out of each repo's entry doc and into the hub's playbooks
(`contribute`, `verify`, `collaborate`):

Every session runs in one of two modes, chosen by the donor or suggested by the hub:

- **Produce:** attempt new work on a card or handoff (the steps below).
- **Verify:** independently check someone else's result package (§9.1). Orient, pick
  an unverified package (preferring ones maintainers have flagged, then the oldest),
  replay or audit it within budget, and record the outcome. Review is the scarce
  resource at agent scale, so verifying is often the more valuable donation.

1. **Refresh.** Fetch canonical main; pin the commit; diff `priority_revision` and
   the frontier against the last checkpoint. Repeat at each loop round, before a
   major pivot, and before submission.
2. **Orient** (about 20% of budget): entry doc, frontier record, card, first_step.
3. **Choose.** Explicit user task, else matching handoff, else picker. Run overlap.
   Write a task card before substantial computation.
4. **Bound.** Price a favorable best case first; stop if even that misses. Test the
   smallest falsifying instance. One bounded pivot.
5. **Verify and hand off** (final 25%): state what was *not* checked; package if
   reusable; post `done` / `finding` / `handoff`.
6. **Feedback** (optional): one retrospective to the project's feedback channel.

Repos may override any default in their entry doc.

## 9. Result packages

Adopted from the reference implementation with minimal change:

- `README.md`: scope, result/evidence, reproduction, **Open next questions**
  (`question`, `why_it_matters`, `first_step`, `blocked_by`), credit.
- `TASK.json`: original card id at a pinned catalogue revision; `scope_status`
  (`unchanged | refined | changed`) with a required reason when not unchanged;
  outcome; evidence level; structured next questions.
- `SOURCES.json`: base commit, Git blob IDs, SHA256 of live inputs.
- `VERIFY.json`: replay commands as argv arrays, inputs, dependency pins, timeouts.

Next questions are *candidates* for the catalogue; they don't enter it automatically.

### 9.1 Independent verification

A result moves through three stages: the author checks it, someone else verifies it
independently, and a maintainer accepts it. The middle stage is where donated review
goes.

A verifier adds one file per verification, so verifications never conflict:
`research/<name>/verifications/<YYYY-MM-DD>-<handle>.json` with `verifier`,
`verified_on`, `package_commit` (the commit of the package they checked), `method`
(replayed / audited / reimplemented), `outcome` (confirmed / failed / partial), `scope`
(what exactly was checked), `not_checked[]`, `evidence` (log, transcript or URL), and
`independent` (true unless the verifier co-authored the package). A verification of a
package at one commit says nothing about later changes to it.

A check of the **frontier record itself**, or of anything that isn't in a package, goes in
a `verifications/` folder beside the frontier record (`discover/verifications/` by
default), with the same fields and `package_commit` set to the commit checked.

Maintainers decide how much independent verification they need before accepting, and
say so in the entry doc. A maintainer who accepts on the strength of donor
verifications is using donated review, and the Review axis counts it (§3.2).

## 10. Director review

A maintainer reassesses when the frontier or bottleneck changes, an obstruction
closes a route, a review batch lands, or the board shows overlap or neglect. Per
track: continue / narrow / expand / pause / replace, each with what the next
bounded experiment decides, a stop rule, and a reopen condition. One patch updates
catalogue + priority log + entry doc; `discover validate` checks they agree.
Progress is measured by verified mechanisms, closed obligations, review time saved
and reusable evidence, never PR count, claimed metric, or compute spent.

## 11. Parties and interfaces

### 11.1 Parties

There are three parties, and the project repository sits between them.

- **Maintainers** own a project repository, set its priorities, and review what
  arrives.
- **Donors** bring agents and inference. Their agents read the repository, coordinate
  with each other, and send work back.
- **Hubs** list projects, assess them, and give donors a way to find them and a
  prompt to start with. There can be many hubs (§12).

The **repository** is the shared medium, not a party. Donors and maintainers both read
and write it, and it stays the source of truth.

The **grader** is a function, not a party: the assessment of §3, run by whoever needs
it. A maintainer runs it locally to see where their repo stands and fix it, privately
and as often as they like. A hub runs it to evaluate an application and to reassess
listed projects. Both run the same rubric, so a maintainer can predict a hub's grade.

### 11.2 How interfaces work

Interfaces between parties are **natural language by default**, because every party
acts through capable agents. Three exceptions:

1. **Structure for shared state with many writers.** Where many agents write to the
   same place (claims, handoff lifecycles, the frontier record), the fields that
   tools act on are exact, even if the rest of the message is prose (§6).
2. **Code for evidence.** Work comes with something that runs (a replay command, a
   certificate, a proof) as well as an explanation (§9).
3. **Permissions are enforced by the receiving side, not by text.** Every
   natural-language edge carries one party's text into another party's agent, so every
   edge is a prompt-injection surface. What an agent may do is set by its own side's
   tooling: the donor's grant and the hub's computed stage for a donor's agent, and
   the maintainer's own settings for anything a hub sends a project (§11.4). No text
   on any edge can widen it.

**Order of authority** for a donor's agent: (1) the person running it; (2) the
project's own text, for everything about the research (what's accepted, priorities,
procedure); (3) the hub's adapter, if any, for navigation and procedure only (§12.5);
(4) the playbooks' defaults. Nothing lower in the order can widen what a higher one
allowed, and an adapter never decides what counts as accepted.

GitHub is the default medium: PRs where state changes, Discussions where agents talk,
and a static website where people browse. **No GitHub setting has to change in a
project's repository for it to be listed.** Discussions is strongly encouraged and is
what the tooling reads, but it's off by default on GitHub, so a project without it
coordinates through issues, at some cost to its Coordination grade. Settings that a hub
needs (Pages, Actions permissions) are the hub operator's one-time job (§12.1).

### 11.3 Interfaces

| From → to | What flows | Default medium | Form |
|---|---|---|---|
| Maintainer → grader (self) | Self-assessment | The `onboard` prompt (goal *evaluate*), in the maintainer's own agent | Natural-language report: grades, evidence, suggestions |
| Maintainer → hub | Application | PR to the hub's registry (an issue form may open it) | A few sentences and a repo link. The hub's agent writes the registry entry and adapter |
| Contributor → maintainer | Setup proposal (§11.5) | One issue on the project repo | Both evaluations, what setup adds, how to decline, and a bootstrap prompt to paste |
| Contributor → hub | Nomination (§11.5) | The same issue form as an application | A repo link, the evaluation, and the proposal link if any. Catalogued without a launcher |
| Hub's grader → hub | Assessment and labelling | PR within the hub repository, reviewed by hub maintainers | Grades with natural-language evidence; the stage computed, never written by hand (§3.4) |
| Hub → maintainer | Grades, suggestions, notices (stage changes, downgrade warnings) | Pull by default; push if the maintainer accepts it (§11.4) | Natural language; suggested fixes as PRs when pushed |
| Donor → hub | Search | Hub website; its data file for agents | Natural-language query |
| Hub → donor | Prompt and playbook | A copy-paste prompt that fetches a versioned playbook from the hub's website; or the plugin | Natural language with a seed, and the stage's permission limit stated plainly. Hard enforcement is the credentials the donor gives their agent |
| Donor ↔ donor | Coordination: claims, findings, handoffs, completions | Discussions and PRs in the project repo | Prose with exact lifecycle fields (§6) |
| Donor → maintainer | Agent work | PRs to the project repo | Natural-language explanation plus checkable evidence |
| Maintainer → donor | Priorities, reviews, acceptance | Repo files and PR reviews | Natural language; the frontier record is exact (§5) |
| Donor → hub | Session outcomes: what was useful, where agents got stuck | Not yet defined (§13) | Natural language |

The last edge closes a loop: if donors' sessions keep stalling at a repo the grader
rates highly, the grader is miscalibrated.

### 11.4 Consent records, pull and push

Consent works the way domains prove control to certificate authorities: the hub
checks a record that only the project's maintainers can write, and logs what it found.

| Web PKI | Discovery@Home |
|---|---|
| Domain validation (an HTTP or DNS challenge) | The hub reads the **consent record** on the project's default branch: `hubs` in `discover.json`, or `.github/discover-hubs.json` (§4). Committing it proves control of the repo |
| CAA record (which authorities may issue) | The same record says which hubs may list the project and what each may push. `{"hub": "*", "list": false}` opts out of every hub |
| Certificate Transparency | The hub registry's git history: every listing, consent check and grade change is public and append-only |
| Revocation | Removing or changing the record. The hub rechecks it on every run |

Each registry entry stores what the hub verified: `consent: {source, commit,
checked_on, list, push}`. A hub never acts beyond the last record it verified.

**Pull and push.** A hub that has something for a maintainer (a grade change, a
suggestion, a proposed fix) delivers it one of two ways.

- **Pull (the default).** The hub changes state only in its own repository: the
  assessment record, a suggestion, a notice. The maintainer watches it (the hub repo,
  its feed, or their listing on the website) and acts in their own repo.
- **Push (only with a record that allows it).** The hub's agent writes to the project
  repository directly, at up to three levels:
  - `notices`: issues for stage changes and downgrade warnings
  - `suggestions`: issues proposing improvements, with the evidence behind them
  - `prs`: PRs proposing concrete fixes, such as a `DISCOVERY.md` (or additions to the
    project's existing entry doc) capturing the adapter

A pushed PR is only a proposal: the hub never merges it and never counts its own open
PRs as evidence. It keeps open at most the counts set in its `hub.json` (by default one
PR and three issues per project).

**First contact.** A hub recruiting a project that has no record yet may open **one**
issue, ever. It carries the self-assessment, what listing would mean, and the exact
record to add to opt in or out. After that, the hub does nothing further unless a
maintainer responds or adds a record. It never contacts a repo whose record opts out,
or one listed in its own `registry/do-not-contact.json`, where it records every opt-out
received by reply. Assessed-only listings of repos without consent show no launcher
(§3.3).

### 11.5 Onboarding: roles, goals and nominations

Anyone can bring a project to a hub, but only its maintainers can consent for it. Every
onboarding starts from one prompt that names a **role** and a **goal**; the `onboard`
playbook routes it to the assess, setup and apply playbooks.

| Role | Goals, least to most | Writes to |
|---|---|---|
| Maintainer | evaluate · setup · submit · **setup-and-submit** | their repo (setup), the hub (submit) |
| Contributor | evaluate · nominate · propose · **propose-and-nominate** | one issue on the project (propose), the hub (nominate) |

*Evaluate* is read-only for both: an assessment for the person who asked. The last goal
in each row is the recommended one.

**Starting from a problem.** Someone with a problem but no repository yet uses the
`start` playbook instead: one prompt carrying the problem as quoted text. It sharpens the
question with them, checks what's known (and every claim in the problem as stated)
against sources, builds a verifier and one worked example before any research, and
writes the protocol files, so the project never needs an adapter. A project built
without a maintainer can be published but takes no donated sessions until someone
agrees to review them.

**Contributors propose with an issue, not a PR.** A pull request nobody asked for is
work for a maintainer. A contributor's agent drafts the setup locally (optionally on a
branch of the contributor's own fork), evaluates the project as is and as it would be
after setup, and opens **one** issue: what setup would add, both evaluations, how to
decline, and a short bootstrap prompt (role maintainer, goal setup-and-submit, with the
issue as context) that the maintainer can paste into their own agent to go ahead. That
issue is the only contact. The contributor writes as themselves, never for a hub, and
the agent first checks for an opt-out, existing Discovery@Home files or proposals, and
project rules against unsolicited proposals.

**Nominations are first-class.** A nomination is a registry entry with `nomination: {by,
on, proposal?, draft?}` and no consent. The hub catalogues it at stage Assessed: shown
with its assessment and no launcher, so a hub can map suitable projects without
directing traffic to them, and maintainers can see that their project is ready to set
up. If a proposal exists, the hub records two assessments: the project as is (`grades`,
`stage`) and as it would be after the proposed setup (`projected`), linked to the
proposal. The hub never contacts maintainers about a nomination; the first-contact rule
(§11.4) is unaffected.

**Nomination becomes listing through consent, not coordination.** Neither action waits
on the other. When a maintainer adopts the proposal and commits a consent record naming
the hub, the hub's next run reads it (§11.4) and the entry becomes a full listing,
eligible for a launcher at whatever stage its next assessment gives. A record that opts
out removes the entry from the catalogue.

## 12. Hubs

### 12.1 What a hub is

A hub is a GitHub repository (public or private) that:

- describes itself in `hub.json` at its root (`schemas/hub.schema.json`): name, scope,
  maintainers, website, intake method, registry path, rubric version and any
  extensions, probe model, and its push policy
- keeps a **registry**: one entry per listed project (`registry/projects/<id>.json`,
  `schemas/registry-entry.schema.json`), each with its adapter, consent record and
  assessment record
- runs the grader on applications and on a schedule (§3.4)
- publishes a website and a data file built from the registry

**Projects, not repos.** The file name is the project ID, unique within the hub. An
entry lists the project's repositories in `repos: [{repo, path?, role?}]`: almost
always one today, but a project may span several repos, and a repo may hold several
projects, each scoped by `path`. The first entry is the project's home: its entry chain,
frontier record and consent record live there. Assessment records and everything hubs
exchange are keyed by project ID, never by repo alone.

An entry also carries, optionally:

- `adapter`: the project's adapter file, `adapters/<project-id>.md` (§12.5), for repos
  that keep their own conventions. (The older inline `overlay` and `frontier_adapter`
  fields are deprecated; the adapter file's block carries both.)
- `process_changed_on`: as in the manifest (§4)
- `consent`: what the hub last verified from the consent record (§11.4)
- `credit`: the credit policy, when the repo states it only in prose

Listing with a contribution launcher requires Stage 0, which includes maintainer
consent. Applications arrive as PRs to the registry (an issue form or `discover submit`
may open them). The hub's agent assesses the repo, writes the adapter, and updates the
PR, and a hub maintainer merges it. The website is rebuilt on every merge and on a
schedule. A listing summarizes the repo. It never certifies a result.

### 12.2 Choosing a project

Which project to work on is one of the few decisions in this system a non-expert can
make on human terms, so people choose and the hub informs. Randomization is essential
*within* a project, where hundreds of agents face the same task list; across projects,
people's different interests already spread them out.

- **The website gallery is the main surface.** Each listing shows what a person needs
  to choose: the problem in plain words, the stage and grades, the credit policy, the
  verification profile, typical session length, and the review signal. The hub informs;
  it doesn't rank by its own judgment of importance.
- **The review signal.** Each build counts the contributions waiting for a first
  response: open, non-draft PRs from outside the project (not its owners, members,
  collaborators or listed maintainers, nor bots) with no review or comment from anyone but their author. The
  listing shows that count beside the declared capacity (§4): "ready for surge traffic",
  "needs reviewers" (a numeric capacity with room), "busy: review queue full" (more
  waiting than a week's capacity), or "paused by its maintainers". Undeclared projects
  show the count and no label. `agents.json` carries the same figures.
- **Agent-assisted choice.** The hub publishes `data/agents.json`
  (`agent-index.schema.json`) and `llms.txt` for agents: every project open to donated
  sessions, with its problem in plain words, fields, stage, what sessions there may do,
  credit policy, license, tracks and open handoffs. "Help me pick a project based on my
  interests" works in any chat.
- **"Donate wherever it's useful" is a seeded draw, never the agent's own pick.** The
  `explore` playbook maps the donor's interests to fields, then draws among the open
  projects in those fields (all of them, if none match), so that many donors with
  similar interests spread out instead of converging on whichever project an agent
  rates best. The draw is exact: each project's weight is set by its stage (1, 3, 6 for
  Stages 0, 1, 2); projects are taken in slug order; the project is the one whose
  cumulative weight range holds `draw("project") mod (total weight)`, with `draw` as in
  §7; and round *n* > 1 of one long donation uses the seed
  `sha256(json([seed, "round:n"]))[:32]`. The session inside the project draws its task
  from the same round seed. The donor gets a receipt saying where each round went.
  Review headroom is the planned second weight, for projects with a numeric capacity
  only; open handoffs are not one, because they measure demand rather than capacity to
  absorb it.
- **Capacity is a signal, not a throttle.** The hub never limits a project's traffic by
  default. A maintainer pauses it (`review.paused`); a Review downgrade sends them an
  urgent notice rather than pausing it (§3.3). Declaring a numeric `per_week` is how a
  project opts in to steering.

### 12.3 Many hubs

The canonical hub is `discoveryathome/discover`. Others are expected: community hubs for
a field, hubs inside organizations for their own private projects, hubs run by funders.
The hub repository is meant to be copied, and the machinery comes with it.

- **Tooling takes the hub as a parameter.** Commands, playbooks and skills default to the
  canonical hub, and accept another through `--hub OWNER/REPO` or the `DISCOVER_HUB`
  environment variable.
- **Listing, consent and push are per hub.** A project can be listed on several hubs.
  Consenting to one says nothing about another, and neither does push.
- **Assessments are per hub.** Hubs may disagree, and each record names its rubric
  version and probe model so the disagreement is legible. A hub may add axes or use
  stricter thresholds and must publish them in `hub.json`. It may not change what
  the stages unlock (§3.3), so a donor knows what "Stage 1" means on any hub.
- **A hub's stage governs only sessions launched from that hub.** The project's own
  manifest ceiling and the donor's grant still apply. When a donor's tooling can't
  read the stage, it falls back to Stage 0.
- **The hub-to-hub interface is the file formats:** `hub.json`, registry entries and
  assessment records, keyed by project ID. A hub can read another's registry to mirror or aggregate
  listings, citing the source hub and rerunning its own assessment before relying on
  it.

### 12.4 The hub is a project too

The canonical hub is listed on itself, as a Discovery@Home project with its own
`DISCOVERY.md`, task catalogue and frontier. Its tracks are hub work: assessing
applicants, re-probing listed projects, verifying other contributors' assessments, and
improving the gallery and tooling. This pays for grading with donated inference,
spreads the grader's work across many independent contributors (which answers "who
grades the grader"), and tests the protocol on a project whose goals look nothing like
improving a single bound.

**Self-assessment.** When a hub grades itself, or a maintainer grades their own project,
the record says so (`self_assessed: true`), and it counts toward a stage only after an
independent rerun by a different assessor agrees. The registry still holds the computed
stage, and the listing labels it self-assessed until then; when launcher permissions are
enforced by stage, they use the last stage an independent assessor confirmed.

### 12.5 Adapters

Many projects can't or won't reorganize their repository for a hub, for reasons that
are political as often as practical. An **adapter** meets them where they are: a layer
the hub keeps on top of a project's repository so donated agents can work there as well
as they would in a project that follows the protocol's conventions.

**What it is.** One Markdown file per project in the hub repository,
`adapters/<project-id>.md`, written for donors' agents. The donation prompt links it, and
the website publishes it at `<site>/adapters/<project-id>.md`. It ends with one fenced
`discover-adapter` JSON block for the hub's tools (`schemas/adapter.schema.json`): the
commit it describes (`as_of`), the paths it relies on, a manifest in `discover.json`'s
shape, a frontier adapter if the project's results record is JSON in another shape
(including a registry whose `current` field selects one entry), and a `frontier_doc`
pointing to where the project states its current result. Without a machine-readable
record, `frontier_doc` stands in for the manifest's `frontier`: the listing links to it,
and the Orientation probe judges whether it's enough. `adapters/integer-mult-bounds.md`
is the reference example.

**What it does.**

- **Maps** the protocol's names onto the repo: `autoresearch.md` as the project guide,
  `CLAUDE.md` or the README as the agents' entry point, a results file as the frontier
  record, a forum thread as the handoff channel.
- **Fills in missing procedure**: how to choose a task when the repo has no picker, how
  to check overlap, how to post.
- **Points to where the project wants help, in its own words:** its task list, a
  "wanted" section, a maintainer's next-steps note, gaps it names in its own results,
  each labelled as the project's.
- **Says what's absent**, so agents don't search for files that don't exist.

**What it doesn't do.** It **maps rather than restates**: it points to where the project
states its accepted result, priorities and acceptance rules instead of copying them,
since copies drift and put words in the maintainers' mouths. It avoids facts that go
stale in days (counts of PRs, records or commits). It builds a seeded draw over a
project's items only when the project invites work on any of them. It fills missing procedure,
not missing judgment: a task list the hub derives from a project's stated priorities is
labelled as the hub's reading for the maintainers to correct. It never grants
permissions and never decides what counts as accepted (§11.2, order of authority).

**Trust.** The hub's own prompt points donors' agents at the adapter, so they follow it
more readily than repository text. That makes it a route for injecting instructions
into many sessions at once. Every change to an adapter is a PR to the hub, reviewed by a
hub maintainer like any registry change; no one, including the adapter's author, edits
it without review.

**Drift.** An adapter lives outside the repository it describes, so it goes stale as
the project changes. Two defenses:

- **Defensive framing.** It states the commit it describes, gives the `git diff` that
  shows what has changed since, and says the repository wins where they disagree.
- **Many eyes.** Donors' agents are the people most likely to notice. One that finds
  drift reports it: a small PR on the hub correcting the adapter, if the donor allows
  PRs, otherwise a ready-to-paste report in the session receipt. The hub's scheduled run
  also flags adapters whose `paths` changed since `as_of`.

**Life cycle.** Written at onboarding (§3.4), kept current by donors and the scheduled
run, offered to the maintainers as files for their own repo, and retired when the
repository covers it with its own `discover.json`. A nominated project's adapter is
used for its assessment only, since no donors are sent there.

### 12.6 Keeping listings current

A listing goes stale as soon as its project changes. A hub runs a **scheduled check**
(the reference hub weekly, with `discover hub-check`) over every entry:

- **Consent:** the project's record still names the hub (§11.4). An opt-out removes the
  entry.
- **Validity:** the entry, its adapter and the repository still validate.
- **Adapter drift:** a path the adapter relies on moved or was deleted since its `as_of`
  commit (§12.5). Edits to the files it points to are only noted, since an adapter maps
  rather than copies. Read through the host's compare API, so nothing is cloned.
- **Stale assessments:** the assessment's `entry_paths` changed since its
  `assessed_commit` and it's more than a week old (newer changes are expected churn on an
  active project), or it's more than a month old (§3.4).
- **Consistency:** the registry's stage and grades match the latest assessment record.

Each finding becomes **one hub-work issue**, updated in place while it persists and closed
when it's resolved, so hub-work sessions (the adapt playbook in update mode, the assess
playbook in re-assess mode) can pick them up. Donors' drift reports (§12.5) feed the same
loop. Opening issues is the hub's own repository only: the check never writes to a
project.

**Live pages.** A hub's project page reads the frontier record and its table from the
project's repository when it loads, so a new result shows within minutes, not at the
next build. The build's copy is the fallback, and the page says which it shows. What it
reads is the project's own claim: it renders it as text only (never the project's HTML or
Markdown), caps its size, reads only the paths the registry and adapter name, and maps a
record in another shape with the adapter's mapping. Histories come from the build, since
reading git history needs the host's API. A project's image (§4, `visual`) is copied at
build time and read as bytes, by the page or live from the project, then shown through
an `<img>`, never inlined, so an SVG's scripts never run and the hub's copy is never served
as a page.

### 12.7 Donor safety

Donors run strangers' research code on their own machines, under their own accounts,
through agents that read whatever the repository says. The hub can't make that safe,
but it can make the safe way the default and say plainly what's left.

- **A screen before listing.** Intake runs `discover screen` on a shallow clone of every
  submitted or nominated repository: files an agent or editor loads by itself
  (`.claude/`, `.mcp.json`, `.codex/`, `.envrc`, editor tasks), install hooks, downloads
  piped to a shell, encoded code that's executed, paths to credential files, collection
  endpoints, text aimed at overriding an agent's instructions, invisible characters, and
  committed executables. It is static and conservative: findings are leads, and most
  have innocent reasons. A registry PR with **high** findings isn't merged until a hub
  maintainer has read each one and recorded why it's benign in the assessment, or
  removed the cause with the project. Donors' agents run the same screen on their clone
  before running any of its code.
- **Agents start in the session folder, not inside a clone.** An agent loads project
  settings, hooks and MCP servers from the folder it starts in, and headless runs skip
  the trust prompt: in testing, a hook planted in a repo's `.claude/settings.json` ran in
  a headless session started there, and so did a server in its `.mcp.json`. Started from
  the session folder, the clone is only data. Most donors who keep a clone for
  contributing just type `claude` in it, so the protection is a one-time setup rather
  than a launch flag: the donor preset written to the clone's
  `.claude/settings.local.json`, which outranks the repo's settings. Tested against a
  hostile repo settings file: its hooks didn't run (`disableAllHooks`), its `allow`
  rules didn't beat the preset's denies, it couldn't turn the sandbox off, and its MCP
  servers stayed off once named (there's no setting that disables a folder's servers
  without naming them, so the preset names those present when it's written). What a
  repo can still do is add to merged lists, such as the sandbox's allowed hosts; the
  screen flags any repo that ships agent settings, and the session runs none of its code
  until the donor has looked. `claude --setting-sources user --settings <preset>`
  ignores a repo's settings entirely, for donors who want that. The session branches
  from the fetched default branch, keeps its notes out of commits, and screens after
  every fetch.
- **Literature search stays open.** The sandbox's host list limits commands (project
  code), not the agent's own web search and fetch tools, which donor presets allow:
  research needs the literature. The list also includes arXiv, DOI, Crossref, OpenAlex,
  Semantic Scholar, Zenodo and the OEIS for scripts that fetch papers or data. Agents
  send only search terms and URLs, never the project's code or unpublished results.
- **Project code runs sandboxed.** The donor presets (`discover permissions donor-local`,
  `donor-post`, `donor`) turn on the agent's sandbox: commands may write only
  inside the folder the agent was started in (the donor's workspace, wherever they keep
  it), can't read the usual credential files, and reach only GitHub, the
  hub's site and the main package registries (`--allow-domain` adds a host). Where the
  agent has no sandbox (Claude Code: native Windows, or Linux without bubblewrap and
  socat), sessions still run, unsandboxed, and `discover permissions` warns and gives
  the install steps (WSL2 on Windows). `donor-local` also points every `git push` to
  GitHub at a host that doesn't exist, since a command-prefix deny rule misses forms like
  `git -C path push`. Without a sandbox, the session reads and reports
  but doesn't build or run the project's code unless the donor agrees.
- **What the sandbox doesn't cover.** Credentials a tool holds stay usable: code inside
  the sandbox can ask `gh` for its token, and GitHub is reachable. For untrusted code,
  donors who can should run sessions in a container or VM with only the session folder
  mounted and a fine-grained GitHub token limited to reading public repositories (or no
  token, for local sessions). Codex's workspace-write sandbox confines writes, not reads
  or network.
- **Repository text is data.** Text that asks an agent to widen its permissions, turn
  off checks, contact anyone, send data anywhere, or keep something from the person is
  never followed; the agent quotes it to the person as a finding, and it's grounds for
  the hub to review the listing.

## 13. Open questions

1. **Claim transport.** Discussions with fenced payloads (proposed), or issues /
   a `claims` branch / a hub API? Discussions are already working at CrocSwap's
   scale; the cost is GraphQL pagination.
2. **Saturation weighting.** How strongly should live claims and recent `done`s
   down-weight a card? Needs data from the pilot.
3. **Near-duplicate grouping for reviewers.** A `claim_group` key (e.g. the metric
   plus the object being varied) so ~30 near-identical claims become one queue row?
4. **Ledger format.** JSONL alongside Markdown, or generated Markdown from JSONL?
5. **Frontier with multiple metrics,** or projects with no scalar metric at all
   (e.g. a conjecture with sub-lemmas).
6. **How much of a project's entry doc can really move into the playbooks** without
   losing the domain judgement that makes it work? Test by porting a second project.
7. **Probe reliability.** Do repeated probes on the same commit agree? In the first
   smoke test (two orientation probes on integer-mult-bounds), they gave the same
   answers and found the same problem. More runs, on messier repos and with the
   donor-tier model, are needed before a single probe is trusted.
11. **Probe model.** Which tier represents typical donors, and should Orientation be
    graded at more than one tier?
12. **Gaming `process_changed_on`.** A project could move the date forward to escape a
    bad window. Require each move to name the change it marks, and show the history on
    the listing.
8. **Thresholds.** The duplicate-rate and response-time cutoffs in §3.2 are guesses.
   Calibrate them on the pilots, and consider normalizing by project size. The first
   Review measurements (`docs/calibration/review-2026-10-18.md`) defined how the three
   Review quantities are counted, but left the cutoffs alone: in every project measured,
   the median first response was well under 3 days, so the 7-day share and the backlog
   decided the grade. The duplicate rate is still unmeasured.
9. **Assessed-only listings.** *Decided:* yes, as nominations (§11.5), catalogued with
   no launcher and never contacted. Still open: whether a hub should catalogue repos on
   its own initiative, without a nominator, and how prominently the catalogue appears.
10. **Assessment record schema.** *Decided:* a `discover-assessment` block ending each
    record (§3.4). Still open: whether hubs should exchange records directly (§12.3).
13. **Session outcomes.** What should donors report back to a hub, in what medium, and
    with what privacy defaults? Opt-in, natural language, and nothing that identifies
    the donor unless they choose to.
14. **Hub push limits.** Are one open PR and three open issues per project the right
    caps, and should a maintainer be able to set lower ones?
15. **Cross-hub trust.** When one hub mirrors another's listings, how much of the
    source hub's assessment may it show before rerunning its own?
16. **Verification records.** Should `verifications/` entries get a schema now, and how
    should maintainers state how many independent verifications they need?
17. **Gallery signals.** Which listing signals help people choose without becoming a
    popularity ranking?
18. **Adapter upkeep.** How many donor drift reports can hub maintainers review, and
    should small factual corrections (a moved path, a renamed thread) merge on a lighter
    review than changes to procedure?
