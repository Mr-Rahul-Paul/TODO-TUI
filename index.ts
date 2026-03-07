import { createCliRenderer, Box, Text } from "@opentui/core"

// ─── Data ────────────────────────────────────────────────────────────────────

type Priority = "high" | "mid" | "low"
type Task = { id: number; text: string; class: string; priority: Priority; done: boolean; date: string }

const CLASSES = ["all", "classwork", "intern", "personal", "research"]
const CLASS_COLORS: Record<string, string> = {
  all:       "#4a9eff",
  classwork: "#a78bfa",
  intern:    "#facc15",
  personal:  "#4ade80",
  research:  "#f87171",
}

const SOCIAL_LINKS = [
  { icon: "⌥", label: "github.com/you" },
  { icon: "◈", label: "portfolio.dev" },
  { icon: "◉", label: "@you on x" },
]

let tasks: Task[] = [
  { id: 1, text: "Set up CI pipeline",          class: "intern",    priority: "high", done: false, date: "today"  },
  { id: 2, text: "Finish DSA assignment – trees",class: "classwork", priority: "mid",  done: false, date: "Mar 09" },
  { id: 3, text: "Read chapter 4 – OS concepts", class: "classwork", priority: "low",  done: false, date: "Mar 10" },
  { id: 4, text: "Draft intern project proposal",class: "intern",    priority: "high", done: false, date: "Mar 08" },
  { id: 5, text: "Update portfolio README",      class: "personal",  priority: "low",  done: false, date: "Mar 12" },
  { id: 6, text: "Review PR from teammate",      class: "intern",    priority: "mid",  done: false, date: "today"  },
  { id: 7, text: "Write unit tests for auth",    class: "research",  priority: "mid",  done: false, date: "Mar 11" },
  { id: 8, text: "Set up opentui project",       class: "personal",  priority: "low",  done: true,  date: "Mar 06" },
  { id: 9, text: "Submit internship application",class: "intern",    priority: "high", done: true,  date: "Mar 04" },
]

// ─── State ───────────────────────────────────────────────────────────────────

type Panel = "classes" | "tasks"

let activePanel: Panel = "tasks"
let selectedClass = 0   // index into CLASSES
let selectedTask  = 0   // index into visible tasks

// ─── Helpers ─────────────────────────────────────────────────────────────────

function visibleTasks(): Task[] {
  const cls = CLASSES[selectedClass]
  return cls === "all" ? tasks : tasks.filter(t => t.class === cls)
}

function classCounts(): number[] {
  return CLASSES.map(cls =>
    cls === "all" ? tasks.length : tasks.filter(t => t.class === cls).length
  )
}

function priorityColor(p: Priority): string {
  return p === "high" ? "#f87171" : p === "mid" ? "#facc15" : "#4ade80"
}

// ─── Rendering ───────────────────────────────────────────────────────────────

function renderClassesPanel(focused: boolean) {
  const counts = classCounts()
  return Box(
    {
      flexDirection: "column",
      borderStyle: "rounded",
      borderColor: focused ? "#4a9eff" : "#2a2a2a",
      width: 24,
      height: "50%",
      padding: 0,
    },
    // header
    Text({ content: focused ? "[1] Classes" : " 1  Classes", fg: focused ? "#4a9eff" : "#555555" }),
    // items
    ...CLASSES.map((cls, i) =>
      Box(
        {
          flexDirection: "row",
          gap: 1,
          bg: i === selectedClass ? "#1a1f2e" : undefined,
        },
        Text({ content: "●", fg: CLASS_COLORS[cls] }),
        Text({
          content: cls.padEnd(12),
          fg: i === selectedClass ? "#ffffff" : "#888888",
          bold: i === selectedClass,
        }),
        Text({
          content: String(counts[i]).padStart(2),
          fg: i === selectedClass ? "#4a9eff" : "#444444",
        }),
      )
    ),
  )
}

function renderSocialPanel(focused: boolean) {
  return Box(
    {
      flexDirection: "column",
      borderStyle: "rounded",
      borderColor: "#2a2a2a",
      width: 24,
      height: "50%",
    },
    Text({ content: " 2  Links", fg: "#555555" }),
    ...SOCIAL_LINKS.map(s =>
      Box(
        { flexDirection: "row", gap: 1 },
        Text({ content: s.icon, fg: "#555555" }),
        Text({ content: s.label, fg: "#666666" }),
      )
    ),
  )
}

function renderTasksPanel(focused: boolean) {
  const visible = visibleTasks()
  const cls = CLASSES[selectedClass]
  const doneCount = visible.filter(t => t.done).length

  // clamp cursor
  if (selectedTask >= visible.length) selectedTask = Math.max(0, visible.length - 1)

  const pending = visible.filter(t => !t.done)
  const done    = visible.filter(t => t.done)

  const taskRow = (t: Task, globalIdx: number) => {
    const isSel = focused && globalIdx === selectedTask
    const checkbox = t.done ? Text({ content: "✓", fg: "#4ade80" }) : Text({ content: " ", fg: "#555555" })
    return Box(
      {
        flexDirection: "row",
        gap: 1,
        bg: isSel ? "#0f1b2d" : undefined,
        borderLeft: isSel ? { size: 1, color: "#4a9eff" } : undefined,
      },
      Text({ content: "[", fg: "#444" }),
      checkbox,
      Text({ content: "]", fg: "#444" }),
      Text({
        content: t.text.padEnd(38),
        fg: t.done ? "#555555" : isSel ? "#ffffff" : "#cccccc",
        strikethrough: t.done,
      }),
      Text({ content: t.priority, fg: priorityColor(t.priority), bold: true }),
      Text({ content: t.date.padStart(8), fg: "#555555" }),
    )
  }

  let allRows: ReturnType<typeof taskRow>[] = []
  let idx = 0
  if (pending.length > 0) {
    allRows.push(
      Text({ content: `  ● Pending  ${pending.length}`, fg: "#666666" }) as any
    )
    for (const t of pending) allRows.push(taskRow(t, idx++))
  }
  if (done.length > 0) {
    allRows.push(
      Text({ content: `  ✓ Done     ${done.length}`, fg: "#555555" }) as any
    )
    for (const t of done) allRows.push(taskRow(t, idx++))
  }

  return Box(
    {
      flexDirection: "column",
      borderStyle: "rounded",
      borderColor: focused ? "#4a9eff" : "#2a2a2a",
      flex: 1,
      height: "100%",
    },
    Box(
      { flexDirection: "row", gap: 1 },
      Text({ content: focused ? "[3] Tasks —" : " 3  Tasks —", fg: focused ? "#4a9eff" : "#555555" }),
      Text({ content: cls, fg: "#a78bfa" }),
      Text({ content: `   ${doneCount}/${visible.length} done`, fg: "#444444" }),
    ),
    ...allRows,
    Text({ content: "  + press 'a' to add…", fg: "#333333" }),
  )
}

function renderStatusBar() {
  const hints = [
    ["j/k", "navigate"],
    ["Space", "toggle"],
    ["a", "add"],
    ["d", "delete"],
    ["Tab", "switch panel"],
    ["q", "quit"],
  ]
  return Box(
    { flexDirection: "row", gap: 2, height: 1, bg: "#0d0d0d" },
    ...hints.map(([key, label]) =>
      Box(
        { flexDirection: "row", gap: 1 },
        Text({ content: `[${key}]`, fg: "#ffffff", bg: "#1a1a1a" }),
        Text({ content: label, fg: "#555555" }),
      )
    ),
  )
}

// ─── Main ────────────────────────────────────────────────────────────────────

const renderer = await createCliRenderer({ exitOnCtrlC: true })

function render() {
  renderer.root.clear()
  renderer.root.add(
    // title bar
    Box(
      { flexDirection: "row", gap: 2, height: 1, bg: "#111111" },
      Text({ content: "◆ todo-tui", fg: "#4a9eff", bold: true }),
      Text({ content: "Tasks", fg: "#ffffff" }),
      Text({ content: "Stats", fg: "#444" }),
      Text({ content: "Config", fg: "#444" }),
    ),

    // main area
    Box(
      { flexDirection: "row", flex: 1, gap: 0 },
      // left column
      Box(
        { flexDirection: "column", width: 26 },
        renderClassesPanel(activePanel === "classes"),
        renderSocialPanel(false),
      ),
      // right column
      renderTasksPanel(activePanel === "tasks"),
    ),

    // status bar
    renderStatusBar(),
  )
}

// ─── Keyboard ────────────────────────────────────────────────────────────────

renderer.on("keydown", (e: { key: string }) => {
  const visible = visibleTasks()

  if (e.key === "Tab") {
    activePanel = activePanel === "tasks" ? "classes" : "tasks"

  } else if (e.key === "j" || e.key === "ArrowDown") {
    if (activePanel === "tasks") {
      selectedTask = Math.min(selectedTask + 1, visible.length - 1)
    } else {
      selectedClass = Math.min(selectedClass + 1, CLASSES.length - 1)
      selectedTask = 0
    }

  } else if (e.key === "k" || e.key === "ArrowUp") {
    if (activePanel === "tasks") {
      selectedTask = Math.max(selectedTask - 1, 0)
    } else {
      selectedClass = Math.max(selectedClass - 1, 0)
      selectedTask = 0
    }

  } else if (e.key === " ") {
    // toggle done
    const t = visible[selectedTask]
    if (t) t.done = !t.done

  } else if (e.key === "d") {
    // delete selected
    const t = visible[selectedTask]
    if (t) tasks = tasks.filter(x => x.id !== t.id)

  } else if (e.key === "q") {
    renderer.destroy()
    process.exit(0)
  }

  render()
})

// initial render
render()