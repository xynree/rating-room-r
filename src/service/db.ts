import { invoke } from '@tauri-apps/api'

export async function getTable(tableName: string) {
  return invoke(tableName).catch((error) => {
    console.error(error)
    return []
  })
}

export async function getItem(id: number):Promise<Item> {
  return await invoke('get_item', { id }).catch(() => null) as Promise<Item>
}

export async function getItems():Promise<Item[]>{
  return await invoke('get_items')
}
