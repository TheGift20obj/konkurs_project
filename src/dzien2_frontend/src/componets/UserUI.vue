<template>
    <div class="p-4">
      <div class="flex space-x-2 mb-4">
        <button class="bg-blue-600 text-white rounded px-4 py-2 hover:bg-blue-700" @click="show_report">Wyświetl awarie</button>
        <button class="bg-blue-600 text-white rounded px-4 py-2 hover:bg-blue-700" @click="add_report_show">Dodaj awarie</button>
      </div>
  
      <div v-if="showing" class="mt-4">
        <div v-if="show_value" class="grid gap-4">
            <div v-for="(report, index) in reports" :key="index">
                <div v-if="userHasAccess(report)" class="bg-stone-300 p-4 rounded drop-shadow-xl">
                    <p>{{ report.nazwa }}
                    <button class="bg-blue-600 text-white rounded px-4 py-2 ml-2 hover:bg-blue-700" @click="hide_report(index)">Wybierz</button>
                    </p>
                </div>
            </div>
        </div>
  
        <div v-else class="bg-white p-6 rounded-lg shadow-md max-w-2xl mx-auto">
          <p class="text-lg font-semibold flex items-center">Awaria:&nbsp;&nbsp;
            <span v-if="editingIndex !== report_index">  {{ report.nazwa }}  </span>
            <input v-if="editingIndex === report_index" v-model="editedReports[report_index]" class="border border-blue-600 p-2 w-full mt-1 rounded">
          </p>
          <p class="mt-2">Właściciel:&nbsp;&nbsp;{{ report.owner.username }}</p>
          <p>Kontakt (gmail):&nbsp;&nbsp;{{ report.owner.gmail }}</p>
          
          <div v-for="(komentarz, index) in report.komentarze" :key="index" class="bg-stone-300 p-4 rounded mt-4 drop-shadow-xl">
            <p class="flex items-center">Komentarz:&nbsp;&nbsp;
              <span v-if="editingComment !== index"> 
                <span v-if="!this.temp_edit_comments[index]"> {{ komentarz }} </span>
                <span v-else>
                    <span v-if="(komentarz == this.temp_edit_comments[index].komentarz || this.temp_edit_comments[index].komentarz == 'null') && this.temp_edit_comments[index].option != 'delete'"> {{ komentarz }} </span>
                    <span class="text-blue-600" v-if="komentarz != this.temp_edit_comments[index].komentarz && this.temp_edit_comments[index].komentarz != 'null' && this.temp_edit_comments[index].option == 'add'"> {{ this.temp_edit_comments[index].komentarz }} <q class="italic text-sm no-quotes">nie zapisano</q></span>
                    <span class="text-red-600" v-if="this.temp_edit_comments[index].komentarz != 'null' && this.temp_edit_comments[index].option == 'delete'"> {{ this.temp_edit_comments[index].komentarz }} <q class="italic text-sm no-quotes text-red-600 line-through">komentarz do usunięcia</q></span>
                </span>
              </span>
              
              <input v-if="editingComment === index" v-model="editComment" class="border border-blue-600 p-2 w-full mt-1 rounded">
            </p>
            <div class="flex space-x-2 mt-2" v-if="editingIndex === report_index && editingComment !== index">
              <button class="bg-blue-600 text-white rounded px-4 py-2 hover:bg-blue-700" @click="edit_comment(index)">Edytuj</button>
            </div>
            <div class="flex space-x-2 mt-2" v-if="editingComment === index">
              <button class="bg-blue-600 text-white rounded px-4 py-2 hover:bg-blue-700" @click="addToQueue(index, 'add')">Zapisz</button>
              <button class="bg-gray-600 text-white rounded px-4 py-2 hover:bg-gray-700" @click="cancelEditComment(index)">Anuluj</button>
              <button class="bg-red-600 text-white rounded px-4 py-2 hover:bg-red-700" @click="addToQueue(index, 'delete')">Usuń</button>
            </div>
          </div>
  
          <div class="flex space-x-2 mt-4 flex-wrap">
            <button v-if="editingIndex !== report_index" class="bg-blue-600 text-white rounded px-4 py-2 hover:bg-blue-700" @click="edit_report(report_index)">Edytuj</button>
            <button v-if="editingIndex === report_index" class="bg-blue-600 text-white rounded px-4 py-2 hover:bg-blue-700" @click="addComment">Dodaj komentarz</button>
            <button v-if="editingIndex === report_index" class="bg-blue-600 text-white rounded px-4 py-2 hover:bg-blue-700" @click="saveEdit(report_index)">Zapisz</button>
            <button v-if="editingIndex === report_index" class="bg-gray-600 text-white rounded px-4 py-2 hover:bg-gray-700" @click="declineEdit">Anuluj</button>
            <button v-if="editingIndex === report_index" class="bg-red-600 text-white rounded px-4 py-2 hover:bg-red-700" @click="delete_report(report_index)">Usuń</button>
          </div>
  
          <input v-if="editingIndex === report_index" v-model="newComment" class="border border-blue-600 p-2 w-full mt-2 rounded" type="text" placeholder="Dodaj nowy komentarz">
        </div>
      </div>
  
      <div v-if="adding" class="flex flex-col items-center mt-4">
        <p class="text-lg font-semibold flex items-center">Awaria:&nbsp;&nbsp;
            <input v-model="newReport" class="border-2 border-blue-600 p-4 w-full max-w-md rounded" type="text" placeholder="podaj nazwę awarii">
        </p>
        
        <button class="bg-blue-600 text-white rounded px-4 py-2 mt-2 hover:bg-blue-700" @click="add_report">Zgłoś</button>
      </div>
    </div>
  </template>

  <script>
  import { dzien2_backend } from 'declarations/dzien2_backend/index';
  
  class Awaria {
      constructor(nazwa, komentarze = [], owner) {
          this.nazwa = nazwa;
          this.komentarze = komentarze;
          this.owner = owner;
      }
  }

  class Item {
    constructor(index, komentarz, option) {
        this.index = index;
        this.komentarz = komentarz;
        this.option = option;
    }
  }
  
  export default {
      data() {
          return {
              newReport: "",
              show_value: false,
              adding: false,
              showing: false,
              reports: [],
              report: new Awaria("", this.$parent.user),
              editingIndex: -1,
              editingComment: -1,
              editedReports: [],
              temp_comments: [],
              temp_edit_comments: [],
              newComment: "",
              editComment: "",
              report_index: -1
          };
      },
      methods: {
          async show_report() {
              this.report_index = -1;
              this.showing = true;
              this.adding = false;
              await this.fetchReports();
              this.show_value = true;
          },
          async hide_report(index) {
              this.report_index = index;
              this.show_value = false;
              this.report = this.reports[index];
              this.reports = [];
          },
          async add_report_show() {
              this.showing = false;
              this.adding = true;
              this.temp_comments = [];
              this.editingIndex = -1;
          },
          async delete_report(index) {
              this.report_index = -1;
              await dzien2_backend.usun_awarie(index);
              this.report = new Awaria("", this.$parent.user);
              this.showing = false;
              this.adding = false;
              this.temp_comments = [];
              this.editingIndex = -1;
          },
          async edit_report(index) {
              this.editedReports[this.report_index] = this.report.nazwa;
              this.temp_comments = [];
              this.editingIndex = index;
          },
          async edit_comment(index) {
              this.editComment = this.report.komentarze[index];
              this.editingComment = index;
          },
          async addComment() {
              this.temp_comments.push(this.newComment);
              this.newComment = "";
          },
          async saveEdit(index) {
              await dzien2_backend.edytuj_awarie(index, this.editedReports[index]);
              for (const comment of this.temp_comments) {
                  await dzien2_backend.dodaj_komentarz(index, comment);
              }
              for (const item of this.temp_edit_comments) {
                if (item) {
                  if (item.option == 'add') {
                    await this.saveComment(item.index, item.komentarz)
                  }
                  else if (item.option == 'delete')
                  {
                    await this.deleteComment(item.index)
                  }
                }
                  
              }
              await this.fetchReports();
              this.report = this.reports[this.report_index];
              this.reports = [];
              this.editingComment = -1;
              this.temp_comments = [];
              this.editingIndex = -1;
              this.editComment = "";
          },
          async declineEdit() {
              this.temp_edit_comments = [];
              await this.fetchReports();
              this.report = this.reports[this.report_index];
              this.reports = [];
              this.editingComment = -1;
              this.temp_comments = [];
              this.editingIndex = -1;
              this.editComment = "";
          },
          async add_report() {
              await dzien2_backend.dodaj_awarie(this.newReport, this.$parent.user);
              this.newReport = "";
          },
          async fetchReports() {
              this.reports = await dzien2_backend.odczytaj_awarie();
              this.editedReports = new Array(this.reports.length).fill("");
          },
          userHasAccess(report) {
              return this.$parent.user.username === report.owner.username || this.$parent.user.role === 'admin';
          },
          async saveComment(index, comment) {
              await dzien2_backend.edytuj_komentarz(this.report_index, index, comment);
              //this.editingComment = -1;
              //await this.fetchReports();
          },
          async cancelEditComment(index) {
              this.temp_edit_comments[index] = new Item(-1, "null", "none")
              this.editingComment = -1;
              this.editComment = "";
          },
          async deleteComment(index) {
              await dzien2_backend.usun_komentarz(this.report_index, index);
              //this.editingComment = -1;
              //await this.fetchReports();
          },
          async addToQueue(index, option)
          {
              this.temp_edit_comments[index] = new Item(index, this.editComment, option)
              this.editingComment = -1;
              this.editComment = "";
          }
      }
  }
  </script>