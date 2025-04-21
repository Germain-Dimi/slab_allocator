# 🧱 slab_alloc

Un mini **slab allocator** écrit en **Rust `no_std`**, conçu pour les environnements bas-niveau ou embarqués.  
Ce projet démontre comment allouer et libérer efficacement de la mémoire pour des objets de taille fixe **sans utiliser le heap dynamique classique** (`Box`, `Vec`, etc.).

---

## 🚀 Fonctionnalités

- ✅ Compatible `#![no_std]`
- 📦 Allocation rapide d'objets de taille fixe
- 🧠 Réutilisation mémoire sans fragmentation
- 🧪 Tests unitaires en mode `std`

---

## 🔧 Utilisation

### Création d’un slab :

```rust
use slab_alloc::Slab;

let mut slab = Slab::new();

#Allocation
let obj = slab.allocate(42).unwrap();
assert_eq!(*obj, 42);

#Libération:
let ptr = obj as *mut _;
slab.deallocate(ptr);

##Tests
cargo test
