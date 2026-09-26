# Independent reviewer comprehension protocol

This protocol measures whether a new reviewer can understand the backup mismatch,
complete the first-run challenge, and state the product boundary from the public materials.
It is separate from automated reproduction and is not a measure of market
adoption or security assurance.

## Participant packet

Give each participant only the repository URL or exact source archive, the
exact commit, the README, and the website URL or local serving command. Do not
provide a private walkthrough, expected answers, ERC-8350 explanation, or an
interpretation of `BAD_PREVIOUS_STATE` before their first answer.

## Session

1. Record the start time with timezone. Ask the participant to open Home.
2. Let them navigate freely. Record whether they can state the task in the
   first 10 seconds, their first action, the challenge completion time, and the
   first confusion. Do not prompt them toward an answer.
3. Ask these two questions verbatim and preserve the answers unedited:
   - What does MemoryLineage check before an agent resumes from a private snapshot?
   - What does MemoryLineage explicitly not prove about the memory or the agent?
4. Ask them to find the canonical head, complete the Home challenge, explain
   the difference between “CHECK PASSED” and “Hold Backup 1”, open the
   `BAD_PREVIOUS_STATE` disclosure, verify the original bundle, tamper one
   commitment, restore the bundle, and run the independent CLI command.
5. Record each task result and command output in the
   [external developer report](external-developer-report.md). Do not fill a
   report on someone else's behalf.

A successful first-run result is unaided completion in about one minute and a
clear explanation that the evidence check passed while the older backup was
held from continuing the latest history. A correct first answer describes
continuity with the canonical predecessor
and configured authorization. A correct second answer excludes semantic truth
or safety of private memory and correctness of agent reasoning. A claim that
the product detects malicious memory is a clarity failure; record it as such
and improve the product copy before repeating the study. Two passing reports
can support a narrow reviewer-comprehension claim, not production adoption or
an error rate estimate.
