#include <Python.h>
#include <math.h>

static PyObject *
short_next_prime(PyObject *obj, PyObject *args)
{
    long long x, i, j, _lim;

    if (!PyArg_ParseTuple(args, "O", &obj)) {Py_RETURN_NONE; return;}
    x = PyLong_AsLongLong(obj);
    if (PyErr_Occurred()) {Py_RETURN_NONE; return;}

    for (i = x + 1;;i++) {
        _lim = sqrt(i) + 1;
        for (j = 2; j <= _lim; j++) {
            if (j == _lim) return Py_BuildValue("l", i);
            if (i % j == 0) break;
        }
    }
}

static PyMethodDef methods[] = {
    {"short_next_prime", (PyCFunction) short_next_prime, METH_VARARGS, "Get next prime after specified number."},
    {NULL, NULL, 0, NULL}
};

static struct PyModuleDef c_api_bindings_module = {
    PyModuleDef_HEAD_INIT,
    "c_api_bindings",
    NULL,
    -1,
    methods
};

PyMODINIT_FUNC
PyInit_c_api_bindings(void)
{
    return PyModule_Create(&c_api_bindings_module);
}
