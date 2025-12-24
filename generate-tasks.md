# generate-tasks.md

## Purpose
Break down a Product Requirements Document (PRD) into granular, actionable tasks that can be tracked and completed iteratively. This guide shows how to create a task hierarchy using the `bd` (Beads) tracking system and prepare the work for implementation.

---

## Step‑by‑Step Process

1. **Load the PRD**  
   - Open the PRD file (e.g., `feature1-prd.md`) in your editor.  
   - Review the scope, user stories, technical approach, and success metrics.

2. **Identify Epics and Major Components**  
   - For each high‑level feature or subsystem, create an **Epic** ID in Beads.  
   - Example epic IDs: `bd-a3f8`, `bd-b2e1`, etc.

3. **Create Tasks Under Each Epic**  
   - For every epic, generate one or more **Task** IDs (e.g., `bd-a3f8.1`, `bd-a3f8.2`).  
   - Each task should correspond to a discrete piece of work that delivers part of the feature.

4. **Decompose Tasks into Sub‑tasks**  
   - Break tasks further into **Sub‑tasks** (e.g., `bd-a3f8.1.1`, `bd-a3f8.1.2`).  
   - Sub‑tasks are typically implementation‑level steps such as “Add schema”, “Write CRUD handlers”, “Add unit tests”, etc.

5. **Populate Task Descriptions**  
   - For each task/sub‑task, write a short description that includes:  
     - **What** needs to be done  
     - **Why** it is needed (link to the PRD section)  
     - Acceptance criteria or definition of done  
   - Example format:  
     ```
     Task: bd-a3f8.1
     - Implement data model for User
       - Sub‑task: bd-a3f8.1.1 - Define schema in `schema.rs`
       - Sub‑task: bd-a3f8.1.2 - Add migration script
       - Sub‑task: bd-a3f8.1.3 - Write unit tests
     ```

6. **Track Progress in Beads**  
   - Use the `bd` CLI to create and manage the hierarchy:  
     - `bd create bd-a3f8 "User data model"` (creates the epic)  
     - `bd add bd-a3f8.1 "Implement schema"` (adds a task)  
     - `bd add bd-a3f8.1.1 "Define schema"` (adds a sub‑task)  
   - Mark tasks as **in_progress**, **blocked**, or **done** using `bd update <id> --status=<state>`.

7. **Generate a Roadmap**  
   - Once all epics, tasks, and sub‑tasks are entered, the Beads system can output a visual roadmap.  
   - Use `bd status` or export to markdown/HTML for review.

8. **Link Tasks Back to the PRD**  
   - In each PRD file, add a reference to the corresponding Beads IDs so reviewers can see the implementation plan.  
   - Example snippet:  
     ```markdown
     ## Implementation Plan
     - Task `bd-a3f8.1` – “Implement data model for User” (see Beads for full sub‑task breakdown)
     ```

---

## Example Workflow

```mermaid
flowchart TD
    A[Feature Idea] --> B[PRD (feature1-prd.md)]
    B --> C["Break into Epics"]
    C --> D[bd-a3f8: User Auth]
    D --> E[bd-a3f8.1: Add JWT]
    E --> F[bd-a3f8.1.1: Create token struct]
    E --> G[bd-a3f8.1.2: Implement encode/decode]
    D --> H[bd-a3f8.2: Add Refresh Token Flow]
    H --> I[bd-a3f8.2.1: Store refresh tokens]
    H --> J[bd-a3f8.2.2: Add token revocation API]
    C --> K[bd-b2e1: Front‑end Integration]
    K --> L[bd-b2e1.1: Update login component]
    K --> M[bd-b2e1.2: Add route guards]
```

- **Epics**: `bd-a3f8`, `bd-b2e1`  
- **Tasks**: `bd-a3f8.1`, `bd-a3f8.2`, `bd-b2e1.1`, `bd-b2e1.2`  
- **Sub‑tasks**: `bd-a3f8.1.1`, `bd-a3f8.1.2`, etc.

---

## Using `bd` Effectively

| Command | Description |
|---------|-------------|
| `bd list` | Show all open tickets. |
| `bd ready` | Find available work (unclaimed tickets). |
| `bd create <id> "<title>"` | Create a new epic/ticket. |
| `bd add <parent-id> "<title>"` | Add a task/sub‑task under a parent. |
| `bd update <id> --status=in_progress` | Mark a ticket as being worked on. |
| `bd update <id> --status=done` | Mark a ticket as completed. |
| `bd close <id>` | Close a ticket once merged. |
| `bd status` | Display current status hierarchy. |

---

## Checklist Before Moving to Implementation

- [ ] All epics, tasks, and sub‑tasks have been entered in Beads.  
- [ ] Each task includes a clear description and acceptance criteria.  
- [ ] Links from the PRD to the corresponding Beads IDs are present.  
- [ ] Statuses are set appropriately (e.g., `in_progress` for current work).  
- [ ] Dependencies between tasks are identified and documented.  

Once the checklist is satisfied, you can proceed to the **Iterative Implementation** phase, picking the next sub‑task, implementing it, and marking it complete before moving on.

---  

*Document version:* 1.0  
*Last updated:* $$(date +"%Y-%m-%d")
