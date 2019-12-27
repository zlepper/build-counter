import { Injectable } from '@angular/core';

export interface LocalStorageState {
  authToken: string;
}

interface ValueWrapper<T> {
  value: T;
}

interface IStorage {
  setItem(key: string, value: string): void;

  getItem(key: string): string | null;

  removeItem(key: string): void;
}

class FakeStorage implements IStorage {
  private readonly data = new Map<string, string>();

  getItem(key: string): string | null {
    const content = this.data.get(key);
    if (typeof content !== 'undefined') {
      return content;
    }
    return null;
  }

  removeItem(key: string) {
    this.data.delete(key);
  }

  setItem(key: string, value: string) {
    this.data.set(key, value);
  }
}

@Injectable({
  providedIn: 'root',
})
/**
 * An abstraction over local storage, that can actually save objects
 * Also handles localStorage not being available (Safari private browsing, i'm looking at you!)
 */
export class LocalStorageService {
  private readonly storage: IStorage;

  constructor() {
    if (typeof localStorage !== 'undefined') {
      this.storage = localStorage;
    } else {
      this.storage = new FakeStorage();
    }
  }

  public set<K extends keyof LocalStorageState>(key: K, value: LocalStorageState[K]) {
    const wrapped: ValueWrapper<LocalStorageState[K]> = {
      value,
    };

    const serialized = JSON.stringify(wrapped);

    this.storage.setItem(key, serialized);
  }

  public get<K extends keyof LocalStorageState>(key: K): LocalStorageState[K] | null {
    const existing = this.storage.getItem(key);

    if (existing == null) {
      return null;
    }

    const wrapped: ValueWrapper<LocalStorageState[K]> = JSON.parse(existing);
    return wrapped.value;
  }

  public remove<K extends keyof LocalStorageState>(key: K) {
    this.storage.removeItem(key);
  }
}
