export interface RouterOption {
  name: string;
  url: string;
  component: Component;
}

export interface Component {
  template: HTMLTemplateElement | HTMLElement,
  mounted: () => void,
  unmounted: () => void,
}

export default class Router {
  app: HTMLElement;
  routes: { [key: string]: RouterOption };
  nowPage: string = '/';
  constructor(router: RouterOption[] = [], app: HTMLElement) {
    for (const element of router) {
      if (element.url === '/') {
        element.component.mounted()
        continue
      }
      element.component.template.remove()
    }
    this.routes = router.reduce((acc: { [key: string]: RouterOption }, curr: RouterOption) => {
      acc[curr.url] = curr;
      return acc;
    }, {});
    this.app = app;
    this.popState()
  }

  public push(url: string, callback?: () => void) {
    window.history.pushState({}, '', `index.html#${url}`);
    this.unmounted(this.routes[this.nowPage])
    this.nowPage = url
    this.mounted(this.routes[url])
    callback && callback()
  }

  private popState() {
    window.addEventListener('hashchange', () => {
      const urlHash = window.location.hash.slice(1);
      console.log('hash', urlHash)
      this.mounted(this.routes[urlHash]);
    });
  }

  // 挂载页面
  private mounted(page: RouterOption) {
    if (!page) return
    this.app.innerHTML = '';
    this.app.appendChild(page.component.template)
    page.component.mounted();
  }

  // 卸载页面
  private unmounted(page: RouterOption) {
    if (!page) return
    this.app.innerHTML = '';
    page.component.unmounted();
  }
}