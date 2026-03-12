# Copied from [here](https://gist.github.com/rowlando/416f41e34fe32840c5634a660df790e1)


# Software Design RFC Process

## Introduction

This RFC (Request for Comments) template is designed to make software design decisions **transparent**, **asynchronous**, and **inclusive**. It serves as a structured way to propose, discuss, and document significant technical decisions before implementation begins.

## When to Use This Template

### Use an RFC when you're proposing changes that:

- **Impact multiple teams or systems** - Cross-cutting concerns that affect other engineers
- **Introduce new architecture patterns** - New frameworks, libraries, or architectural approaches
- **Change existing APIs or interfaces** - Breaking changes or significant API modifications
- **Require significant engineering investment** - Projects taking more than 2-3 engineer-weeks
- **Involve trade-offs with multiple valid solutions** - When the "how" isn't obvious and needs discussion
- **Set technical precedents** - Decisions that will influence future technical choices

**Don't use for:** Small bug fixes, routine maintenance, or well-established patterns your team already uses.

## Why Use This Process

- **Async-first collaboration:** Engineers can review and provide input on their own schedule, accommodating different time zones and work styles
- **Transparent decision-making:** Creates a paper trail of technical decisions, rationale, and trade-offs for future reference
- **Broader input:** Gives subject matter experts across the organisation a chance to weigh in before implementation
- **Reduced rework:** Catches potential issues early, before code is written
- **Knowledge sharing:** Helps spread technical knowledge and context across teams

## Where to Store RFCs

- **Product-specific RFCs:** Store in your product's dedicated Confluence space
- **Cross-cutting RFCs:** Store in the general "Software Engineering" Confluence space
- **Naming convention:** `RFC-YYYY-MM-DD-brief-title` (e.g., "RFC-2025-06-02-api-rate-limiting")

## How to Use This Template

### Step 1: Start with the Problem

Before diving into solutions, clearly articulate the problem you're solving. If you can't explain why this RFC is needed, it might not be needed yet.

### Step 2: Write the Summary First

Draft your 2-3 sentence summary before writing anything else. This forces you to crystallise your thinking and gives reviewers an immediate understanding of what you're proposing.

### Step 3: Fill Out What You Know

Don't wait until you have all the answers. Fill in the sections you're confident about, and use "Open Questions" for areas where you need input.

### Step 4: Share Early for Feedback

Share your draft RFC when it's 70% complete. Early feedback is more valuable than perfect prose. Tag specific people whose expertise you need.

### Step 5: Iterate Based on Comments

Treat comments as collaborative input, not criticism. Update the RFC based on discussions, and don't hesitate to change your approach if better ideas emerge.

### Step 6: Drive to Decision

Set a clear timeline for feedback (typically 1-2 weeks). After incorporating input, call for final approval from key stakeholders. Avoid endless discussion - sometimes "disagree and commit" is necessary.

## Writing Tips

- **Use plain language** - Avoid jargon that might exclude reviewers from other teams
- **Include diagrams** - Visual representations often communicate better than paragraphs
- **Be specific** - Vague proposals lead to vague implementations
- **Show your work** - Explain why you rejected alternatives, don't just list them
- **Update as you learn** - RFCs are living documents during the review process

---

## The Template

### RFC: [Brief Descriptive Title]

**Author:** [Your Name]  
**Date:** [YYYY-MM-DD]  
**Status:** [Draft | In Review | Approved | Implemented | Superseded]  
**Reviewers:** [Tag relevant people/teams]

### Summary

[2-3 sentences describing what you're proposing and why it matters]

### Background & Motivation

What problem are we solving? What's the current state that's driving this need? Include relevant context that reviewers need to understand the problem space.

### Proposal

#### Overview

[High-level description of your proposed solution]

#### Detailed Design

[The meat of your proposal. Include:]

- Architecture diagrams or sketches
- API changes or new interfaces
- Data models or schema changes
- Key algorithms or logic flows
- Integration points with existing systems

#### Examples

[Code snippets, API examples, or usage scenarios that illustrate how this would work in practice]

### Trade-offs & Alternatives

#### Alternatives Considered

- **Option A:** [Brief description and why it was rejected]
- **Option B:** [Brief description and why it was rejected]

#### Trade-offs

**Pros:** [What this approach does well]

**Cons:** [What this approach sacrifices or makes harder]

### Implementation Plan

#### Phases

1. **Phase 1:** [What gets built first]
2. **Phase 2:** [Subsequent phases if applicable]

#### Timeline

[Rough estimates - weeks/months, not days]

#### Success Metrics

[How will we know this worked? Performance improvements, error rate reductions, etc.]

### Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| [What could go wrong?] | [High/Medium/Low] | [How we'll address it] |

### Open Questions

- [ ] [Things you're still uncertain about]
- [ ] [Decisions that need input from specific people/teams]

### Appendix

[Supporting materials: research, benchmarks, links to related docs, etc.]

---

## Review Process

1. **Draft:** Author creates initial RFC
2. **Review:** Share with relevant stakeholders (1-2 weeks for feedback)
3. **Discussion:** Address comments and iterate on the design
4. **Approval:** Key stakeholders sign off
5. **Implementation:** Build it
6. **Retrospective:** Update RFC with lessons learned post-implementation
