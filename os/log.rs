8702f3e4 in ?? ()
8702f3e6 in ?? ()
8702f3e8 in ?? ()
8702f3ea in ?? ()
8702f3ec in ?? ()
8702f3ee in ?? ()
8702f3f2 in ?? ()
8702f3f6 in ?? ()
8702f3fa in ?? ()
8702f3fc in ?? ()
8702f400 in ?? ()
8702f404 in ?? ()
8702f408 in ?? ()
8702f40a in ?? ()
8702f40c in ?? ()
8702f410 in ?? ()
8702f414 in ?? ()
8702f416 in ?? ()
8702f418 in ?? ()
8702f41c in ?? ()

000000008702f3e4 <_ZN4core4sync6atomic11atomic_load17h16d4d25f149ce800E>:
unsafe fn atomic_load<T: Copy>(dst: *const T, order: Ordering) -> T {
    8702f3e4:	7139                	addi	sp,sp,-64
    8702f3e6:	fc06                	sd	ra,56(sp)
    8702f3e8:	f822                	sd	s0,48(sp)
    8702f3ea:	0080                	addi	s0,sp,64
    8702f3ec:	862e                	mv	a2,a1
    8702f3ee:	fcb40fa3          	sb	a1,-33(s0)
    8702f3f2:	fea43423          	sd	a0,-24(s0)
        Acquire => intrinsics::atomic_load_acq(dst),
    8702f3f6:	0ff5f593          	andi	a1,a1,255
    8702f3fa:	4681                	li	a3,0
    8702f3fc:	fca43823          	sd	a0,-48(s0)
    8702f400:	fcb43423          	sd	a1,-56(s0)
    8702f404:	04d58f63          	beq	a1,a3,8702f462 <.LBB5_16+0x2a>
    8702f408:	a009                	j	8702f40a <_ZN4core4sync6atomic11atomic_load17h16d4d25f149ce800E+0x26>
    8702f40a:	4505                	li	a0,1
    8702f40c:	fc843583          	ld	a1,-56(s0)
    8702f410:	06a58b63          	beq	a1,a0,8702f486 <.LBB5_17>
    8702f414:	a009                	j	8702f416 <_ZN4core4sync6atomic11atomic_load17h16d4d25f149ce800E+0x32>
    8702f416:	4509                	li	a0,2
    8702f418:	fc843583          	ld	a1,-56(s0)
    8702f41c:	02a58a63          	beq	a1,a0,8702f450 <.LBB5_16+0x18>


8702f450 in ?? ()
8702f454 in ?? ()
8702f456 in ?? ()
8702f45a in ?? ()
8702f45e in ?? ()
8702f460 in ?? ()
        Acquire => intrinsics::atomic_load_acq(dst),
    8702f450:	fd043503          	ld	a0,-48(s0)
    8702f454:	610c                	ld	a1,0(a0)
    8702f456:	0230000f          	fence	r,rw
    8702f45a:	feb43023          	sd	a1,-32(s0)
    8702f45e:	a009                	j	8702f460 <.LBB5_16+0x28>
    match order {
    8702f460:	a091                	j	8702f4a4 <.LBB5_18+0x16>
        Relaxed => intrinsics::atomic_load_relaxed(dst),


8702f4a4 in ?? ()
8702f4a8 in ?? ()
8702f4aa in ?? ()
8702f4ac in ?? ()
8702f4ae in ?? ()

    8702f4a4:	fe043503          	ld	a0,-32(s0)
    8702f4a8:	7442                	ld	s0,48(sp)
    8702f4aa:	70e2                	ld	ra,56(sp)
    8702f4ac:	6121                	addi	sp,sp,64
    8702f4ae:	8082                	ret


8700e3a4 in ?? ()
8700e3a8 in ?? ()
8700e3aa in ?? ()
8700e3ae in ?? ()
8700e3b0 in ?? ()
8700e3b2 in ?? ()
8700e3b4 in ?? ()

    8700e3a4:	fca43423          	sd	a0,-56(s0)
    8700e3a8:	a009                	j	8700e3aa <_ZN4core4sync6atomic11AtomicUsize4load17h40da77a2f1c21f19E+0x3a>
                }
    8700e3aa:	fc843503          	ld	a0,-56(s0)
    8700e3ae:	7442                	ld	s0,48(sp)
    8700e3b0:	70e2                	ld	ra,56(sp)
    8700e3b2:	6121                	addi	sp,sp,64
    8700e3b4:	8082                	ret


8700ec70 in ?? ()
8700ec74 in ?? ()
8700ec78 in ?? ()
8700ec7a in ?? ()
8700ec7e in ?? ()

    8700ec6c:	708080e7          	jalr	1800(ra) # 8700e370 <_ZN4core4sync6atomic11AtomicUsize4load17h40da77a2f1c21f19E>
    8700ec70:	cf843583          	ld	a1,-776(s0)
    8700ec74:	00b50863          	beq	a0,a1,8700ec84 <.LBB0_84+0x510>
    8700ec78:	a009                	j	8700ec7a <.LBB0_84+0x506>
            crate::relax();
    8700ec7a:	00000097          	auipc	ra,0x0
    8700ec7e:	82a080e7          	jalr	-2006(ra) # 8700e4a4 <_ZN4core4sync6atomic14spin_loop_hint17h2b4f617ce93a07b7E>

8700e4a4 in ?? ()
8700e4a6 in ?? ()
8700e4a8 in ?? ()
8700e4aa in ?? ()
8700e4ac in ?? ()
8700e4b0 in ?? ()



000000008700e4a4 <_ZN4core4sync6atomic14spin_loop_hint17h2b4f617ce93a07b7E>:
pub fn spin_loop_hint() {
    8700e4a4:	1141                	addi	sp,sp,-16
    spin_loop()
    8700e4a6:	e406                	sd	ra,8(sp)
    8700e4a8:	e022                	sd	s0,0(sp)
    8700e4aa:	0800                	addi	s0,sp,16
    8700e4ac:	fffff097          	auipc	ra,0xfffff
    8700e4b0:	79a080e7          	jalr	1946(ra) # 8700dc46 <_ZN4core4hint9spin_loop17h6b56b3aadb093d98E>
    8700e4b4:	a009                	j	8700e4b6 <_ZN4core4sync6atomic14spin_loop_hint17h2b4f617ce93a07b7E+0x12>
}

8700dc46 in ?? ()
8700dc48 in ?? ()
8700dc4a in ?? ()
8700dc4c in ?? ()
8700dc4e in ?? ()
8700dc50 in ?? ()
8700dc52 in ?? ()
8700dc54 in ?? ()

000000008700dc46 <_ZN4core4hint9spin_loop17h6b56b3aadb093d98E>:
/// do anything at all.
///
/// [`core::sync::atomic::spin_loop_hint`]: ../sync/atomic/fn.spin_loop_hint.html
#[inline]
#[unstable(feature = "renamed_spin_loop", issue = "55002")]
pub fn spin_loop() {
    8700dc46:	1141                	addi	sp,sp,-16
            // SAFETY: the `cfg` attr ensures that we only execute this on arm targets
            // with support for the v6 feature.
            unsafe { crate::arch::arm::__yield() };
        }
    }
}
    8700dc48:	e406                	sd	ra,8(sp)
    8700dc4a:	e022                	sd	s0,0(sp)
    8700dc4c:	0800                	addi	s0,sp,16
    8700dc4e:	6402                	ld	s0,0(sp)
    8700dc50:	60a2                	ld	ra,8(sp)
    8700dc52:	0141                	addi	sp,sp,16
    8700dc54:	8082                	ret
	...


8700e4b4 in ?? ()
8700e4b6 in ?? ()
8700e4b8 in ?? ()
8700e4ba in ?? ()
8700e4bc in ?? ()

000000008700e4a4 <_ZN4core4sync6atomic14spin_loop_hint17h2b4f617ce93a07b7E>:
pub fn spin_loop_hint() {
    8700e4a4:	1141                	addi	sp,sp,-16
    spin_loop()
    8700e4a6:	e406                	sd	ra,8(sp)
    8700e4a8:	e022                	sd	s0,0(sp)
    8700e4aa:	0800                	addi	s0,sp,16
    8700e4ac:	fffff097          	auipc	ra,0xfffff
    8700e4b0:	79a080e7          	jalr	1946(ra) # 8700dc46 <_ZN4core4hint9spin_loop17h6b56b3aadb093d98E>
    8700e4b4:	a009                	j	8700e4b6 <_ZN4core4sync6atomic14spin_loop_hint17h2b4f617ce93a07b7E+0x12>
}
    8700e4b6:	6402                	ld	s0,0(sp)
    8700e4b8:	60a2                	ld	ra,8(sp)
    8700e4ba:	0141                	addi	sp,sp,16
    8700e4bc:	8082                	ret




8700ec82 in ?? ()

    8700ec7e:	82a080e7          	jalr	-2006(ra) # 8700e4a4 <_ZN4core4sync6atomic14spin_loop_hint17h2b4f617ce93a07b7E>
        while self.next_serving.load(Ordering::Acquire) != ticket {
    8700ec82:	bff9                	j	8700ec60 <.LBB0_84+0x4ec>
            next_serving: &self.next_serving,


8700ec60 in ?? ()
8700ec64 in ?? ()
8700ec66 in ?? ()
8700ec68 in ?? ()
8700ec6c in ?? ()

```
    8700ec60:	d0043503          	ld	a0,-768(s0)
    8700ec64:	0521                	addi	a0,a0,8
    8700ec66:	4589                	li	a1,2
    8700ec68:	fffff097          	auipc	ra,0xfffff
    8700ec6c:	708080e7          	jalr	1800(ra) # 8700e370 <_ZN4core4sync6atomic11AtomicUsize4load17h40da77a2f1c21f19E>
```

8700e370 in ?? ()
8700e372 in ?? ()
8700e374 in ?? ()
8700e376 in ?? ()
8700e378 in ?? ()
8700e37a in ?? ()
8700e37e in ?? ()
8700e382 in ?? ()
8700e386 in ?? ()
8700e38a in ?? ()

000000008700e370 <_ZN4core4sync6atomic11AtomicUsize4load17h40da77a2f1c21f19E>:
                pub fn load(&self, order: Ordering) -> $int_type {
    8700e370:	7139                	addi	sp,sp,-64
    8700e372:	fc06                	sd	ra,56(sp)
    8700e374:	f822                	sd	s0,48(sp)
    8700e376:	0080                	addi	s0,sp,64
    8700e378:	862e                	mv	a2,a1
    8700e37a:	fea43023          	sd	a0,-32(s0)
    8700e37e:	feb407a3          	sb	a1,-17(s0)
                    unsafe { atomic_load(self.v.get(), order) }
    8700e382:	fcc43c23          	sd	a2,-40(s0)
    8700e386:	00021097          	auipc	ra,0x21
    8700e38a:	e1a080e7          	jalr	-486(ra) # 8702f1a0 <_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h217378281cbea8bbE>

8702f1a0 in ?? ()
8702f1a2 in ?? ()
8702f1a4 in ?? ()
8702f1a6 in ?? ()
8702f1a8 in ?? ()
8702f1ac in ?? ()
8702f1ae in ?? ()
8702f1b0 in ?? ()
8702f1b2 in ?? ()

000000008702f1a0 <_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h217378281cbea8bbE>:
    pub const fn get(&self) -> *mut T {
    8702f1a0:	1101                	addi	sp,sp,-32
    8702f1a2:	ec06                	sd	ra,24(sp)
    8702f1a4:	e822                	sd	s0,16(sp)
    8702f1a6:	1000                	addi	s0,sp,32
    8702f1a8:	fea43423          	sd	a0,-24(s0)
    }
    8702f1ac:	6442                	ld	s0,16(sp)
    8702f1ae:	60e2                	ld	ra,24(sp)
    8702f1b0:	6105                	addi	sp,sp,32
    8702f1b2:	8082                	ret


8700e38e in ?? ()
8700e392 in ?? ()
8700e394 in ?? ()
8700e398 in ?? ()
8700e39c in ?? ()
8700e3a0 in ?? ()
8702f3e4 in ?? ()

000000008700e370 <_ZN4core4sync6atomic11AtomicUsize4load17h40da77a2f1c21f19E>:
                pub fn load(&self, order: Ordering) -> $int_type {
    8700e370:	7139                	addi	sp,sp,-64
    8700e372:	fc06                	sd	ra,56(sp)
    8700e374:	f822                	sd	s0,48(sp)
    8700e376:	0080                	addi	s0,sp,64
    8700e378:	862e                	mv	a2,a1
    8700e37a:	fea43023          	sd	a0,-32(s0)
    8700e37e:	feb407a3          	sb	a1,-17(s0)
                    unsafe { atomic_load(self.v.get(), order) }
    8700e382:	fcc43c23          	sd	a2,-40(s0)
    8700e386:	00021097          	auipc	ra,0x21
    8700e38a:	e1a080e7          	jalr	-486(ra) # 8702f1a0 <_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h217378281cbea8bbE>
    8700e38e:	fca43823          	sd	a0,-48(s0)
    8700e392:	a009                	j	8700e394 <_ZN4core4sync6atomic11AtomicUsize4load17h40da77a2f1c21f19E+0x24>
    8700e394:	fd043503          	ld	a0,-48(s0)
    8700e398:	fd843583          	ld	a1,-40(s0)
    8700e39c:	00021097          	auipc	ra,0x21
    8700e3a0:	048080e7          	jalr	72(ra) # 8702f3e4 <_ZN4core4sync6atomic11atomic_load17h16d4d25f149ce800E>
    8700e3a4:	fca43423          	sd	a0,-56(s0)
    8700e3a8:	a009                	j	8700e3aa <_ZN4core4sync6atomic11AtomicUsize4load17h40da77a2f1c21f19E+0x3a>
                }
    8700e3aa:	fc843503          	ld	a0,-56(s0)
    8700e3ae:	7442                	ld	s0,48(sp)
    8700e3b0:	70e2                	ld	ra,56(sp)
    8700e3b2:	6121                	addi	sp,sp,64
    8700e3b4:	8082                	ret