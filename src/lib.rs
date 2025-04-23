#![no_std]

/// On importe MaybeUninit de `core::mem` pour manipuler de la mémoire non initialisée en toute sécurité.
use core::mem::MaybeUninit;

/// Constante définissant la capacité maximale du slab (le nombre d'éléments pouvant être stockés)
const SLAB_SIZE: usize = 32;

/// Structure principale représentant notre slab allocator générique
pub struct Slab<T> {
    /// Tableau non initialisé de T : utilisé pour stocker les objets alloués
    storage: [MaybeUninit<T>; SLAB_SIZE],
    /// Tableau booléen indiquant quels blocs sont utilisés (`true`) ou libres (`false`)
    used: [bool; SLAB_SIZE],
}

impl<T> Slab<T> {
    /// Crée une nouvelle instance vide du slab allocator
    pub fn new() -> Self {
        Self {
            /// Initialisation "dangereuse" d’un tableau de MaybeUninit
            /// On assume qu’on peut initialiser ce tableau sans le remplir (valide pour MaybeUninit)
            storage: unsafe { MaybeUninit::uninit().assume_init() },
            /// Tous les blocs sont marqués comme libres
            used: [false; SLAB_SIZE],
        }
    }

    /// Tente d’allouer un bloc pour stocker `value`.
    /// Retourne un pointeur mutable vers la valeur stockée, ou `None` s’il n’y a plus de place.
    pub fn allocate(&mut self, value: T) -> Option<&mut T> {
        for i in 0..SLAB_SIZE {
            /// Si le bloc à l’index i est libre
            if !self.used[i] {
                self.used[i] = true; // On le marque comme utilisé
                self.storage[i] = MaybeUninit::new(value); // On y place la valeur

                /// On retourne un pointeur sûr vers la valeur allouée
                return unsafe { Some(&mut *self.storage[i].as_mut_ptr()) };
            }
        }
        /// Aucun bloc libre trouvé : retour None
        None
    }

    /// Libère un bloc précédemment alloué en fournissant un pointeur brut vers la donnée
    pub fn deallocate(&mut self, ptr: *mut T) {
        for i in 0..SLAB_SIZE {
            /// On vérifie que ce bloc est bien utilisé et que le pointeur correspond
            if self.used[i] && self.storage[i].as_ptr() == ptr {
                self.used[i] = false; // On le marque comme libre
                return;
            }
        }
    }
}

