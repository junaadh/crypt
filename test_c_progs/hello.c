int x_global_uninit;

int x_global_init = 1;

static int y_global_uninit;

static int y_global_init = 2;

extern int z_global;

int fn_a(int x, int y);

static int fn_b(int x, int y) { return x + y; }

int fn_c(int x_local) {
  int y_local_uninit;

  int y_local_init = 3;

  x_global_uninit = fn_a(x_local, x_global_init);
  y_local_uninit = fn_a(x_local, y_local_init);
  y_local_uninit += fn_b(z_global, 1);

  return (y_global_uninit + y_local_uninit);
}
