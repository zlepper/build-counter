import { shareReplay } from 'rxjs/operators';

export function sharingIsCaring<T>(buffer = 1) {
  return shareReplay<T>({
    bufferSize: buffer,
    refCount: true,
  });
}
