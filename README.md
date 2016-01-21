# spiderq protocol for Rust

## Summary

Protocol implementation for [spiderq](https://github.com/swizard0/spiderq) server as Rust crate.

## Specifications

### Agreements.

All numeric values (`uint16_t`, `uint32_t` и `uint64_t`) are encoded using network byte order (`big endian`), all byte arrays (`uint8_t[]` -- keys and values) are copied the same ways as it `memcpy` does.

### Ping / Pong

#### Request.

* Request: `Ping()`
* Description: check if connection is established and server is alive.
* Format: <pre>0x0B:uint8_t</pre>
* Valid frame example for `Ping()`: <pre>0B</pre>

#### Reply.

* Reply: `Pong`
* Description: request received, server is OK.
* Format: <pre>0x11:uint8_t</pre>
* Valid frame example for `Pong`: <pre>11</pre>

### Count / Counted

#### Request.

* Request: `Count()`
* Description: get total amount of tasks in queue (`Lend()` requests before block).
* Format: <pre>0x01:uint8_t</pre>
* Valid frame example for `Count()`: <pre>01</pre>

#### Reply.

* Reply: `Counted(total)`
* Description: result received.
* Parameters:
 * `total`: `uint32_t` — pending tasks count.
* Format: <pre>0x01:uint8_t total:uint32_t</pre>
* Valid frame example for `Counted(10)`: <pre>01 00 00 00 0A</pre>

### Add / Added / Kept

#### Request.

* Request: `Add(key, value)`
* Description: add new entry both into kv database and tasks queue. Do nothing if there is already an entry with the same key.
* Parameters:
 * `key`: `uint8_t[]` — new entry key
 * `value`: `uint8_t[]` — new entry value
* Format: <pre>0x02:uint8_t key_length:uint32_t key:uint8_t[] value_length:uint32_t value:uint8_t[]</pre>
* Valid frame example for `Add("cat", "small")`: <pre>02 00 00 00 03 63 61 74 00 00 00 05 73 6D 61 6C 6C</pre>

#### Reply.

* Reply variant: `Added()`
* Description: the entry was successfully added.
* Format: <pre>0x02:uint8_t</pre>
* Valid frame example for `Added()`: <pre>02</pre>

or

* Reply variant: `Kept()`
* Description: the entry was ignored because there is already an entry in database with the same key.
* Format: <pre>0x03:uint8_t</pre>
* Valid frame example for `Kept()`: <pre>03</pre>

### Update / Updated / NotFound

#### Request.

* Request: `Update(key, value)`
* Description: update existing entry in kv database. Do nothing if there is no such entry with the key given.
* Parameters:
 * `key`: `uint8_t[]` — entry key for updating
 * `value`: `uint8_t[]` — new entry value
* Format: <pre>0x03:uint8_t key_length:uint32_t key:uint8_t[] value_length:uint32_t value:uint8_t[]</pre>
* Valid frame example for `Update("cat", "small")`: <pre>03 00 00 00 03 63 61 74 00 00 00 05 73 6D 61 6C 6C</pre>

#### Reply.

* Reply variant: `Updated()`
* Description: the entry was successfully updated.
* Format: <pre>0x04:uint8_t</pre>
* Valid frame example for `Updated()`: <pre>04</pre>

or

* Reply variant: `NotFound()`
* Description: nothing done.
* Format: <pre>0x05:uint8_t</pre>
* Valid frame example for `NotFound()`: <pre>05</pre>

### Lookup / ValueFound / ValueNotFound

#### Request.

* Request: `Lookup(key)`
* Description: lookup existing entry value in kv database.
* Parameters:
 * `key`: `uint8_t[]` — entry key
* Format: <pre>0x09:uint8_t key_length:uint32_t key:uint8_t[]</pre>
* Valid frame example for `Lookup("cat")`: <pre>09 00 00 00 03 63 61 74</pre>

#### Reply.

* Reply variant: `ValueFound(value)`
* Description: the entry was found and it's value is returned.
* Format: <pre>0x0D:uint8_t value_length:uint32_t value:uint8_t[]</pre>
* Valid frame example for `ValueFound("small")`: <pre>0D 00 00 00 05 73 6D 61 6C 6C</pre>

or

* Reply variant: `ValueNotFound()`
* Description: the entry was not found.
* Format: <pre>0x0E:uint8_t</pre>
* Valid frame example for `ValueNotFound()`: <pre>0E</pre>

### Lend / Lent / QueueEmpty

#### Request.

* Request: `Lend(timeout_ms, mode)`
* Description: get next task from the queue. If the task was not returned by client with `Repay()` after `timeout_ms`, it will be automatically put back into the front of the queue. If the queue was empty when `Lend()` arrives, decision is maked according to `mode` parameter:
 * When `mode` == `Block`, the request will be blocked until a task is available.
 * When `mode` == `Poll`, the `QueueEmpty` reply will be immediately returned.
* Parameters:
 * `timeout_ms`: `uint64_t` — timeout in milliseconds. 
 * `mode`: `uint8_t` — empty queue case behaviour: `0x01` for `Block` and `0x02` for `Poll`.
* Format: <pre>0x04:uint8_t timeout_ms:uint64_t mode:uint8_t</pre>
* Valid frame example for `Lend(1000)`: <pre>04 00 00 00 00 00 00 03 E8 01</pre>

#### Reply.

* Reply variant: `Lent(lend_key, key, value)`
* Description: an entry with `key` and `value` is received as a task.
* Parameters:
 * `lend_key`: `uint64_t` — opaque task serial number, it should be returned by cliend in `Repay` or `Heartbeat` requests. 
 * `key`: `uint8_t[]` — task entry key
 * `value`: `uint8_t[]` — task entry value
* Format: <pre>0x06:uint8_t lend_key:uint64_t key_length:uint32_t key:uint8_t[] value_length:uint32_t value:uint8_t[]</pre>
* Valid frame example for `Lent(1, "cat", "small")`: <pre>06 00 00 00 00 00 00 00 01 00 00 00 03 63 61 74 00 00 00 05 73 6D 61 6C 6C</pre>

or

* Reply variant: `QueueEmpty()`
* Description: queue is empty, no tasks to return.
* Format: <pre>0x10:uint8_t</pre>
* Valid frame example for `QueueEmpty()`: <pre>10</pre>

### Repay / Repaid

#### Request.

* Request: `Repay(lend_key, key, changed_value, status)`
* Description: return the previously lent task back to the queue, change task entry value to the given `changed_value` and then modify the task priority according the `status` parameter:
 * `Penalty`: decrease priority (accumulated). The task will be positioned closer to the end of the queue.
 * `Reward`: increase priority (accumulated). The task will be positioned closer to the front of the queue.
 * `Front`: set the maximum priority.
 * `Drop`: drop the task entirely from the queue. Entry will remain in kv database, but no `Lend()` request will return it.
* Parameters:
 * `lend_key`: `uint64_t` — opaque task serial number that has been received by `Lend()`. 
 * `key`: `uint8_t[]` — task entry key that has been received by `Lend()`.
 * `changed_value`: `uint8_t[]` — new entry value.
 * `status`: `uint8_t` — `0x01` for `Penalty`, `0x02` for `Reward`, `0x03` for `Front` and `0x04` for `Drop`. 
* Format: <pre>0x05:uint8_t lend_key:uint64_t key_length:uint32_t key:uint8_t[] changed_value_length:uint32_t changed_value:uint8_t[] status:uint8_t</pre>
* Valid frame example for `Repay(1, "cat", "big", Reward)`: <pre>05 00 00 00 00 00 00 00 01 00 00 00 03 63 61 74 00 00 00 03 62 69 67 02</pre>

#### Reply.

* Reply variant: `Repaid()`
* Description: the task was successfully returned to the queue (or dropped for `status` == `Drop`).
* Format: <pre>0x07:uint8_t</pre>
* Valid frame example for `Repaid()`: <pre>07</pre>

or

* Reply variant: `NotFound()`
* Description: invalid `key` or `lend_key` for task, or this task was already returned to the queue after `timeout_ms`.
* Format: <pre>0x05:uint8_t</pre>
* Valid frame example for `NotFound()`: <pre>05</pre>

### Heartbeat / Heartbeaten

#### Request.

* Request: `Heartbeat(lend_key, key, timeout_ms)`
* Description: modify task timeout that was previously received by `Lend()`, but not yet returned to the queue (with `Repay()` request or automatically due timeout).
* Parameters:
 * `lend_key`: `uint64_t` — opaque task serial number that has been received by `Lend()`. 
 * `key`: `uint8_t[]` — task entry key that has been received by `Lend()`.
 * `timeout_ms`: `uint64_t` — new timeout in milliseconds for the task.
* Format: <pre>0x06:uint8_t lend_key:uint64_t key_length:uint32_t key:uint8_t[] timeout_ms:uint64_t</pre>
* Valid frame example for `Heartbeat(1, "cat", 2000)`: <pre>06 00 00 00 00 00 00 00 01 00 00 00 03 63 61 74 00 00 00 00 00 00 07 D0</pre>

#### Reply.

* Reply variant: `Heartbeaten()`
* Description: timeout was successfully changed.
* Format: <pre>0x08:uint8_t</pre>
* Valid frame example for `Heartbeaten()`: <pre>08</pre>

or

* Reply variant: `Skipped()`
* Description: invalid `key` or `lend_key` for task, or this task was already returned to the queue after `timeout_ms`.
* Format: <pre>0x09:uint8_t</pre>
* Valid frame example for `Heartbeaten()`: <pre>09</pre>

### Stats / StatsGot

#### Request.

* Request: `Stats()`
* Description: get statistics counters values.
* Format: <pre>0x07:uint8_t</pre>
* Valid frame example for `Stats()`: <pre>07</pre>

#### Reply.

* Reply: `StatsGot(count, add, update, lookup, lend, repay, heartbeat, stats)`
* Description: counters values were received.
* Parameters:
 * `count`: `uint64_t` — count of `Count` requests after server startup.
 * `add`: `uint64_t` — count of `Add` requests after server startup.
 * `update`: `uint64_t` — count of `Update` requests after server startup.
 * `lookup`: `uint64_t` — count of `Lookup` requests after server startup.
 * `lend`: `uint64_t` — count of `Lend` requests after server startup.
 * `repay`: `uint64_t` — count of `Repay` requests after server startup.
 * `heartbeat`: `uint64_t` — count of `Heartbeat` requests after server startup.
 * `stats`: `uint64_t` — count of `Stats` requests after server startup.

* Format: <pre>0x0A:uint8_t count:uint64_t add:uint64_t update:uint64_t lookup:uint64_t lend:uint64_t repay:uint64_t heartbeat:uint64_t stats:uint64_t</pre>
* Valid frame example for `StatsGot(1, 2, 3, 4, 5, 6, 7, 8)`: <pre>0A 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00 02 00 00 00 00 00 00 00 03 00 00 00 00 00 00 00 04 00 00 00 00 00 00 00 05 00 00 00 00 00 00 00 06 00 00 00 00 00 00 00 07 00 00 00 00 00 00 00 08</pre>

### Flush / Flushed

#### Request.

* Request: `Flush()`
* Description: force kv database disk synchronizing.
* Format: <pre>0x0A:uint8_t</pre>
* Valid frame example for `Flush()`: <pre>0A</pre>

#### Reply.

* Reply: `Flushed()`
* Description: kv database sync was completed successfully.
* Format: <pre>0x0F:uint8_t</pre>
* Valid frame example for `Flushed()`: <pre>0F</pre>

### Terminate / Terminated

#### Request.

* Request: `Terminate()`
* Description: terminate server `spiderq`.
* Format: <pre>0x08:uint8_t</pre>
* Valid frame example for `Terminate()`: <pre>08</pre>

#### Reply.

* Reply: `Terminated()`
* Description: `spiderq` server was terminated successfully.
* Format: <pre>0x0C:uint8_t</pre>
* Valid frame example for `Terminated()`: <pre>0C</pre>

## License

The MIT License (MIT)

Copyright (c) 2016 Alexey Voznyuk

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.