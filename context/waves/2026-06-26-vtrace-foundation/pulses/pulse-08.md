# Pulse 08: ARCHITECTURE

Settled. Authored `docs/vtrace/ARCHITECTURE.md`: 7 components (basin-network /
corpus / score / tier / gap / cli + docs review layer), scale allocated to the
corpus and gap layers, downward-only dependency direction, data flow, dependencies,
and failure modes (incl. hydrologic-basis-unknown and silent-cross-scale → hold).
Fixed point: removed a potential `corpus→score` cycle. Next: INTERFACES.
