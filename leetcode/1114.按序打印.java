/*
 * @lc app=leetcode.cn id=1114 lang=java
 *
 * [1114] 按序打印
 */

// @lc code=start
class Foo {
    private int mark = 1;
    public Foo() {
        
    }

    public void first(Runnable printFirst) throws InterruptedException {
        synchronized(this){
            while (mark != 1) {
                this.wait();
            }
            // printFirst.run() outputs "first". Do not change or remove this line.
            printFirst.run();
            mark = 2;
            this.notifyAll();
        }
    }

    public void second(Runnable printSecond) throws InterruptedException {
        synchronized(this){
            while (mark != 2) {
                this.wait();
            }
            // printSecond.run() outputs "second". Do not change or remove this line.
            printSecond.run();
            mark = 3;
            this.notifyAll();
        }
    }

    public void third(Runnable printThird) throws InterruptedException {
        synchronized(this){
            while (mark != 3) {
                this.wait();
            }
            // printThird.run() outputs "third". Do not change or remove this line.
            printThird.run();
        }
    }
}
// @lc code=end

