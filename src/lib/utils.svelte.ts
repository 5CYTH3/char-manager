export enum LockableCell {
  None,
  StatsCell,
  StuffCell,
  InventoryCell,
}
export let lockingMutex = $state({ value: LockableCell.None });
