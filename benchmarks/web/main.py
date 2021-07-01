import time

import requests
from matplotlib import pyplot as plt

URL = 'http://0.0.0.0:8000/{}?flight_id=1'
N = 500


def go(kind):
    times = []
    url = URL.format(kind)
    for _ in range(N):
        resp = requests.get(url)
        times.append(float(resp.headers['X-Process-Time']))
        time.sleep(0.01)
    return [*range(N)], times


def main():
    basic_times = go('basic')
    time.sleep(5)
    ffi_times = go('ffi')
    fig, ax = plt.subplots()
    ax.scatter(*ffi_times, label='ffi', s=5)
    ax.scatter(*basic_times, label='basic', s=5)
    plt.xlabel("Номер попытки")
    plt.ylabel("Время (sec)")
    ax.legend()
    fig.savefig("queries.png")


if __name__ == '__main__':
    main()
