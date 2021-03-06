import pickle
import time
import multiprocessing
from multiprocessing import Process, Event
from typing import Dict, Any

from syft import node
from syft.message import SyftMessage, execute_capability, create_handler
from syft.protos import SyftMessageProto

from syft.message.syft_message import SyftMessageProxy


# this is required for pickle to work over multiprocess
class TestCapabilities:
    @staticmethod
    def sum_func(numbers: list) -> int:
        return sum(numbers)


def test_node_message() -> None:
    caps = {"sum": TestCapabilities.sum_func}
    port = 8080
    iface = "0.0.0.0"
    with NodeProcess(iface, port, caps):
        target_addr = f"http://{iface}:{port}"

        any_object = set([1, 2, 3])
        message = SyftMessage(capability="sum", obj=any_object, id="1")
        response = message.send(target_addr)
        assert response == 6


def test_execute_capability() -> None:
    caps = {"sum": TestCapabilities.sum_func}
    port = 8080
    iface = "0.0.0.0"
    with NodeProcess(iface, port, caps):
        target_addr = f"http://{iface}:{port}"

        any_object = set([1, 2, 3])
        answer = execute_capability(target_addr, "sum", any_object)
        assert answer == 6


def test_node_capabilities() -> None:
    caps = {"sum": TestCapabilities.sum_func}
    port = 8080
    iface = "0.0.0.0"
    with NodeProcess(iface, port, caps):
        target_addr = f"http://{iface}:{port}"

        capabilities = node.request_capabilities(target_addr)
        assert capabilities == ["sum"]


# this allows the ability to write with NodeProcess(caps): to auto handle node up/down
class NodeProcess:
    def __init__(
        self,
        iface: str = "0.0.0.0",
        port: int = 50051,
        capabilities: Dict[str, Any] = {},
    ) -> None:
        self.iface = iface
        self.port = port
        self.capabilities = capabilities
        # create an event for non blocking wait in the process
        self.event = Event()

    def __enter__(self) -> None:
        self.startup()

    def __exit__(self, type, value, traceback) -> None:
        self.event.set()
        self.p.join()

    def startup(self) -> None:
        # start the node in a separate process
        self.p = Process(
            target=NodeProcess.start_node,
            args=(self.event, self.iface, self.port, self.capabilities,),
        )
        self.p.start()

        # wait for startup
        time.sleep(1)

    @staticmethod
    def start_node(
        event: Event, iface: str, port: int, capabilities: Dict[str, Any]
    ) -> None:
        node.start(iface, port)
        for name, cap in capabilities.items():
            node.register(name, create_handler(cap))
        while not event.is_set():
            event.wait(1)
