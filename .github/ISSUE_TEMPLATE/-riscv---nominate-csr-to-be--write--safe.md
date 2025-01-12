---
name: "`riscv`: Nominate CSR to be `write` safe"
about: Suggest to make writes of a given CSR safe
title: "`riscv`: make [CSR] write-safe"
labels: ''
assignees: ''

---

**Which CSR do you want to nominate as `write` safe?**
Indicate which CSR you want to be `write` safe. Ex. `mepc`

**Does a CSR write introduce potential memory safety issues in safe code? Please describe.**
A clear and concise justification on why writing to this CSR will **never** introduce memory safety issues in safe code.

**Does a CSR write introduce potential undefined behavior in safe code? Please describe.**
A clear and concise justification on why writing to this CSR will **never** lead to undefined behavior in safe code.

**Does a CSR write invalidate invariants in safe code?**
A clear and concise justification on why writing to this CSR will **never** invalidate invariants in safe code.

**Additional context**
Please feel free to add any other context or screenshots about your request here.
