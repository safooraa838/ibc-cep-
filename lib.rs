#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{pallet_prelude::*, sp_std::vec::Vec};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    /// Stores the tasks which assigned by you
    #[pallet::storage]
    #[pallet::getter(fn tasks)]
    pub type Tasks<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

    /// Stores the tasks which are assigned to you by other users.
    #[pallet::storage]
    #[pallet::getter(fn assigned_tasks)]
    pub type AssignedTasks<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TaskAdded { task: Vec<u8> },
        TaskCompleted { task: Vec<u8> },
        TaskAssigned { task: Vec<u8> },
        AssignedTaskCompleted { task: Vec<u8> },
        Edit { task: Vec<u8> },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        TaskAlreadyAdded,
        TaskNotFound,
        TaskAlreadyAssigned,
        TaskNotAssigned,
    }

    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics," which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Any user can add tasks to themselves.
        /// Check if it is already present in the storage.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn add_task(origin: OriginFor<T>, task: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;

            let mut all_tasks = Tasks::<T>::get(who.clone());

            // Check if task is already present
            ensure!(!all_tasks.contains(&task), Error::<T>::TaskAlreadyAdded);

            all_tasks.push(task.clone());

            // store the new task in the storage
            Tasks::<T>::insert(who, all_tasks);

            Self::deposit_event(Event::<T>::TaskAdded { task });

            Ok(())
        }

        /// User can mark their tasks as complete.
        /// If the task is marked as complete then it automatically deleted from the storage.
        /// Check if this task is present in the storage or not
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn mark_as_complete(origin: OriginFor<T>, task: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;

            let all_tasks = Tasks::<T>::get(who.clone());

            // Check if task is present or not
            ensure!(all_tasks.contains(&task), Error::<T>::TaskNotFound);

            // store the task
            Tasks::<T>::mutate(&who, |tasks| {
                let index = tasks.iter().position(|x| *x == task).unwrap();
                tasks.remove(index);
            });

            Self::deposit_event(Event::<T>::TaskCompleted { task });

            Ok(())
        }

        /// Any user can assign tasks to others.
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn assign_task_to_other(
            origin: OriginFor<T>,
            who: T::AccountId,
            task: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Check if the sender has the task in their task list
            let sender_tasks = Tasks::<T>::get(sender.clone());
            ensure!(sender_tasks.contains(&task), Error::<T>::TaskNotFound);

            // Add the task to the assigned tasks of the target user
            let mut target_assigned_tasks = AssignedTasks::<T>::get(who.clone());
            target_assigned_tasks.push(task.clone());
            AssignedTasks::<T>::insert(who.clone(), target_assigned_tasks);

            Self::deposit_event(Event::<T>::TaskAssigned { task });

            Ok(())
        }

        /// User can mark assigned tasks as complete which are assigned to them by other users.
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn mark_assigned_task_as_complete(
            origin: OriginFor<T>,
            task: Vec<u8>,
        ) -> DispatchResult {
            let receiver = ensure_signed(origin)?;

            // Check if the task is assigned to the receiver
            let receiver_assigned_tasks = AssignedTasks::<T>::get(receiver.clone());
            ensure!(receiver_assigned_tasks.contains(&task), Error::<T>::TaskNotAssigned);

            // Mark the task as complete by removing it from the assigned tasks list
            AssignedTasks::<T>::mutate(&receiver, |tasks| {
                if let Some(index) = tasks.iter().position(|x| *x == task) {
                    tasks.remove(index);
                    Self::deposit_event(Event::<T>::AssignedTaskCompleted { task });
                }
            });

            Ok(())
        }

        /// User can only edit the tasks which are assigned by themselves.
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn edit_task(origin: OriginFor<T>, task: Vec<u8>, new_task: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;

            let mut all_tasks = Tasks::<T>::get(who.clone());

            // Check if task is present or not
            ensure!(all_tasks.contains(&task), Error::<T>::TaskNotFound);

            Tasks::<T>::mutate(&who, |tasks| {
                let index = tasks.iter().position(|x| *x == task).unwrap();
                tasks.remove(index);
            });

            // Get the updated tasks
            all_tasks = Tasks::<T>::get(who.clone());

            // Add the new task
            all_tasks.push(new_task);

            Tasks::<T>::insert(who, all_tasks);

            Self::deposit_event(Event::<T>::Edit { task });

            Ok(())
        }
    }
}